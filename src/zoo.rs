use crate::Currency;
pub use bevy::prelude::*;

pub struct ZooPlugin;
impl Plugin for ZooPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Zoo>()
            .add_event::<ZooBalanceChange>()
            .add_event::<OnZooBalanceChanged>()
            .add_systems(Update, handle_balance_change);
    }
}

#[derive(Resource)]
pub struct Zoo {
    balance: Currency,
}

impl Default for Zoo {
    fn default() -> Self {
        // TEMP: giving an initial balance for testing
        Self { balance: 5000.0 }
    }
}

impl Zoo {
    /// Returns the current balance of the zoo
    pub fn balance(&self) -> Currency {
        self.balance
    }
}

/// Event for systems to request a balance change to the `Zoo`
#[derive(Event)]
pub struct ZooBalanceChange {
    pub amount: Currency,
}

/// Callback event for systems that want to be notified of a balance change
#[derive(Event)]
pub struct OnZooBalanceChanged {
    pub balance: Currency,
}

fn handle_balance_change(
    mut zoo: ResMut<Zoo>,
    mut request_reader: EventReader<ZooBalanceChange>,
    mut callback_writer: EventWriter<OnZooBalanceChanged>,
) {
    for change_balance_event in request_reader.iter() {
        zoo.balance += change_balance_event.amount;

        callback_writer.send(OnZooBalanceChanged {
            balance: zoo.balance,
        });
    }
}
