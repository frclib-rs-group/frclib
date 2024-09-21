/// Only executes the code inside the block if the feature `frc_sim` is enabled.
#[macro_export]
macro_rules! if_sim {
    {$($t:tt)*} => {
        #[cfg(frc_sim)]
        {$($t)*}
    }
}

/// Only executes the code inside the block if the feature `frc_real` is enabled.
#[macro_export]
macro_rules! if_real {
    {$($t:tt)*} => {
        #[cfg(frc_real)]
        {$($t)*}
    }
}

/// Only executes the code inside the block if the feature `frc_dev` is enabled.
#[macro_export]
macro_rules! if_dev {
    {$($t:tt)*} => {
        #[cfg(frc_dev)]
        {$($t)*}
    }
}

/// Only executes the code inside the block if the feature `frc_dev` is not enabled.
#[macro_export]
macro_rules! if_not_dev {
    {$($t:tt)*} => {
        #[cfg(not(frc_dev))]
        {$($t)*}
    }
}

#[macro_export]
macro_rules! team_number {
    () => {
        env!("FRC_TEAM_NUMBER").parse::<u16>().expect("Invalid team number")
    };
}

#[macro_export]
macro_rules! deploy_dir {
    () => {
        env!("FRC_DEPLOY_DIR")
    };
}