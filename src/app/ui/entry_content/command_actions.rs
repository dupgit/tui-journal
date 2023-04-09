use crate::{
    app::{commands::UICommand, App, UIComponents},
    data::DataProvider,
};

pub(crate) fn execute_command<D: DataProvider>(
    command: UICommand,
    ui_components: &mut UIComponents,
    app: &mut App<D>,
) -> anyhow::Result<()> {
    match command {
        UICommand::SaveEntryContent => {}
        UICommand::DiscardChangesEntryContent => {}
        UICommand::FinishEditEntryContent => {}
        _ => unreachable!(
            "{:?} is not implemented for entry content text box",
            command
        ),
    }

    Ok(())
}
