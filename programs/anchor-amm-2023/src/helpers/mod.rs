#[macro_export]
macro_rules! assert_non_zero {
    ($array:expr) => {
        if $array.contains(&0u64) {
            return err!(AmmError::ZeroBalance)
        }
    };
}

#[macro_export]
macro_rules! assert_not_locked {
    ($lock:expr) => {
        if $lock == true {
            return err!(AmmError::PoolLocked)
        }
    };
}

#[macro_export]
macro_rules! assert_not_expired {
    ($expiration:expr) => {
        if Clock::get()?.unix_timestamp > $expiration {
            return err!(AmmError::OfferExpired);
        }
    };
}

#[macro_export]
macro_rules! has_update_authority {
    ($x:expr) => {
        match $x.config.authority {
            Some(a) => {
                require_keys_eq!(a, $x.user.key(), AmmError::InvalidAuthority);
            },
            None => return err!(AmmError::NoAuthoritySet)
        }
    };
}


