use tauri::State;

#[cfg(target_os = "linux")]
use zbus::Connection;

#[tauri::command]
pub async fn get_accent_color() -> Result<String, String> {
    #[cfg(target_os = "linux")]
    {
        let conn = Connection::session().await.map_err(|e| e.to_string())?;
        let proxy = zbus::ProxyBuilder::new_bare(&conn)
            .interface("org.freedesktop.portal.Settings")
            .path("/org/freedesktop/portal/desktop")
            .destination("org.freedesktop.portal.Desktop")
            .unwrap()
            .build()
            .await
            .map_err(|e| e.to_string())?;

        let result: Result<(String,), _> = proxy
            .call("Read", &("org.freedesktop.appearance", "accent-color"))
            .await;

        match result {
            Ok((color,)) => Ok(color),
            Err(_) => Ok(String::new()),
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        Ok(String::new())
    }
}

#[tauri::command]
pub async fn update_unity_launcher_count(count: u32) {
    #[cfg(target_os = "linux")]
    {
        if let Ok(conn) = Connection::session().await {
            if let Ok(proxy) = zbus::ProxyBuilder::new_bare(&conn)
                .interface("com.canonical.Unity.LauncherEntry")
                .path("/com/canonical/Unity/LauncherEntry")
                .destination("com.canonical.Unity")
                .unwrap()
                .build()
                .await
            {
                let _ = proxy
                    .call::<_, ()>(
                        "Update",
                        &(
                            "application://veskto.desktop",
                            [("count", zbus::zvariant::Value::from(count))],
                        ),
                    )
                    .await;
            }
        }
    }
}

#[tauri::command]
pub async fn request_background(auto_start: bool, command_line: String) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        let conn = Connection::session().await.map_err(|e| e.to_string())?;
        let proxy = zbus::ProxyBuilder::new_bare(&conn)
            .interface("org.freedesktop.portal.Background")
            .path("/org/freedesktop/portal/desktop")
            .destination("org.freedesktop.portal.Desktop")
            .unwrap()
            .build()
            .await
            .map_err(|e| e.to_string())?;

        let _ = proxy
            .call::<_, String>(
                "RequestBackground",
                &("", command_line, auto_start),
            )
            .await;
    }

    #[cfg(not(target_os = "linux"))]
    {
        let _ = (auto_start, command_line);
    }

    Ok(())
}
