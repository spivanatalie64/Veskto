use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioNode {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub is_input: bool,
}

#[tauri::command]
pub async fn list_audio_nodes() -> Result<Vec<AudioNode>, String> {
    #[cfg(target_os = "linux")]
    {
        // TODO: implement using pipewire-rs crate
        // For now, return empty list
        Ok(vec![])
    }

    #[cfg(not(target_os = "linux"))]
    {
        Ok(vec![])
    }
}

#[tauri::command]
pub async fn start_virtual_mic(_target_node: u32) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        // TODO: implement using pipewire-rs crate
        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    {
        let _ = _target_node;
        Ok(())
    }
}

#[tauri::command]
pub async fn stop_virtual_mic() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        // TODO: implement using pipewire-rs crate
        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    {
        Ok(())
    }
}

#[tauri::command]
pub async fn start_system_audio() -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        // TODO: implement using pipewire-rs crate
        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    {
        Ok(())
    }
}
