use hdi::prelude::{*, tracing::field::debug};
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Contact {
    pub agent_pub_key: AgentPubKey,
    pub text_number: Option<String>,
    pub whatsapp_number: Option<String>,
    pub email_address: Option<String>,
}
pub fn validate_create_contact(
    action: EntryCreationAction,
    contact: Contact,
) -> ExternResult<ValidateCallbackResult> {
    debug!("-----------------------> validate create contact: {:?}", action);
    // return Ok(ValidateCallbackResult::Invalid("Only the notificant can do this".into()));
    debug!("-----------------------> contact agent: {:?}", contact.agent_pub_key.clone());
    debug!("-----------------------> action agent: {:?}", action.author().clone());
    debug!("-----------------------> same: {:?}", contact.agent_pub_key.clone() == action.author().clone().into());

    if contact.agent_pub_key != action.author().clone().into() {
        return Ok(ValidateCallbackResult::Invalid("Only the notificant can do this".into()));
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_contact(
    action: Update,
    contact: Contact,
    original_action: EntryCreationAction,
    _original_contact: Contact,
) -> ExternResult<ValidateCallbackResult> {
    if (original_action.author().clone() != action.author.clone()) || (action.author.clone() != contact.agent_pub_key) {
        return Ok(ValidateCallbackResult::Invalid("Only the notificant can do this".into()));
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_contact(
    action: Delete,
    original_action: EntryCreationAction,
    _original_contact: Contact,
) -> ExternResult<ValidateCallbackResult> {
    if original_action.author().clone() != action.author.clone() {
        return Ok(ValidateCallbackResult::Invalid("Only the notificant can do this".into()));
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_contact_updates(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(base_address);
    let record = must_get_valid_record(action_hash)?;
    let _contact: crate::Contact = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _contact: crate::Contact = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_contact_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("ContactUpdates links cannot be deleted"),
        ),
    )
}
