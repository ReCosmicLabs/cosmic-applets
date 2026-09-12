// SPDX-License-Identifier: GPL-3.0-only

//! Contador de nao lidas publicado pelos apps no D-Bus: sinal `Update` da interface
//! `com.canonical.Unity.LauncherEntry`, que Discord, Telegram e afins emitem via libunity
//! (`app.setBadgeCount` do Electron no Linux). O KDE le a mesma coisa pro badge da barra.

use cosmic::iced::{self, Subscription, stream};
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use zbus::zvariant::OwnedValue;

/// (id do .desktop sem a extensao, contador; 0 = esconder)
#[derive(Debug, Clone)]
pub struct LauncherEntryUpdate {
    pub desktop_id: String,
    pub count: u32,
}

pub fn launcher_entry_subscription() -> iced::Subscription<LauncherEntryUpdate> {
    Subscription::run_with(std::any::TypeId::of::<LauncherEntryUpdate>(), |_| {
        stream::channel(16, move |mut output| async move {
            loop {
                if let Err(why) = listen(&mut output).await {
                    tracing::warn!(?why, "LauncherEntry: sem escuta no D-Bus");
                }
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            }
        })
    })
}

async fn listen(
    output: &mut futures::channel::mpsc::Sender<LauncherEntryUpdate>,
) -> zbus::Result<()> {
    let connection = zbus::Connection::session().await?;
    let rule = zbus::MatchRule::builder()
        .msg_type(zbus::message::Type::Signal)
        .interface("com.canonical.Unity.LauncherEntry")?
        .member("Update")?
        .build();
    let mut stream = zbus::MessageStream::for_match_rule(rule, &connection, Some(32)).await?;

    while let Some(msg) = stream.next().await {
        let msg = match msg {
            Ok(msg) => msg,
            Err(_) => continue,
        };
        let body = msg.body();
        let Ok((uri, props)): Result<(String, HashMap<String, OwnedValue>), _> =
            body.deserialize()
        else {
            continue;
        };
        // "application://discord.desktop" -> "discord"
        let desktop_id = uri
            .strip_prefix("application://")
            .unwrap_or(&uri)
            .trim_end_matches(".desktop")
            .to_string();
        let visible = props
            .get("count-visible")
            .and_then(|v| bool::try_from(v.clone()).ok())
            .unwrap_or(true);
        let count = props
            .get("count")
            .and_then(|v| i64::try_from(v.clone()).ok())
            .unwrap_or(0);
        let count = if visible && count > 0 {
            u32::try_from(count).unwrap_or(u32::MAX)
        } else {
            0
        };
        // So o `count` importa; sinal sem ele (progress, urgent) nao mexe no badge.
        if props.contains_key("count") || props.contains_key("count-visible") {
            let _ = output.send(LauncherEntryUpdate { desktop_id, count }).await;
        }
    }
    Ok(())
}
