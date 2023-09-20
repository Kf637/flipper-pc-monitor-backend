use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral, PeripheralId};
use futures::stream::StreamExt;
use uuid::Uuid;

pub const FLIPPER_CHARACTERISTIC_UUID: Uuid =
    Uuid::from_u128(0x19ed82ae_ed21_4c9d_4145_228e62fe0000);

pub async fn get_central(manager: &Manager) -> Adapter {
    manager
        .adapters()
        .await
        .unwrap()
        .into_iter()
        .nth(0)
        .unwrap()
}

pub async fn get_flipper(central: &Adapter, id: &PeripheralId) -> Option<Peripheral> {
    for p in central
        .peripherals()
        .await
        .unwrap()
        .iter()
        .filter(|p| p.id() == *id)
    {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("L4te0t") or name.contains("Kf637") or name.contains("Flipper"))
        {
            return Some(p.clone());
        }
    }
    None
}
