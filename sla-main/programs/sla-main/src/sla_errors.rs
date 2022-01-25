use anchor_lang::prelude::*;

#[error]
pub enum SlaErrors {
  #[msg("The user account is not owner of the Mint")]
  UserDoesNotOwnMint,

  #[msg("The user is not the owner of the token specified")]
  TokenPDAMismatch,

  #[msg("The Avatar PDA is not an Avatar account")]
  PDAIsNotAnAvatar,

  #[msg("The Trait PDA is not a Trait account")]
  PDAIsNotATrait,

  #[msg("The Trait type specified is not valid")]
  TraitTypeInvalid,

  #[msg("The Avatar PDA has already been initialized")]
  AvatarAlreadyInitialized,

  #[msg("The Trait PDA has already been initialized")]
  TraitAlreadyInitialized,

  #[msg("The Trait specified cannot be merged")]
  MergeCheckFailed,

  #[msg("The Trait cannot be merged because the Avatar has not been initialized")]
  MergeCheckFailedBecauseAvatarNotInitialized,
}