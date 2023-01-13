set-env STARSHIP_SHELL "elvish"
set-env STARSHIP_SESSION_KEY (to-string (randint 10000000000000 10000000000000000))
# Set up base variables
set-env STARSHIP_JOBS "0"
set-env STARSHIP_STATUS "0"
set-env STARSHIP_DURATION "0"
unset-env STARSHIP_PIPESTATUS

# Define Hooks
var cmd-status-code = 0

fn starship-after-command-hook {|m|
    var error = $m[error]
    if (is $error $nil) {
        set cmd-status-code = 0
    } else {
        try {
            set cmd-status-code = $error[reason][exit-status]
        } catch {
            # The error is from the built-in commands and they have no status code.
            set cmd-status-code = 1
        }
    }
}

# Install Hooks
set edit:after-command = [ $@edit:after-command $starship-after-command-hook~ ]

# Install starship
set edit:prompt = {
    set-env STARSHIP_STATUS $cmd-status-code
    set-env STARSHIP_DURATION (printf "%.0f" (* $edit:command-duration 1000))
    set-env STARSHIP_JOBS $num-bg-jobs
    ::STARSHIP:: prompt --logical-path=$pwd
}

set edit:rprompt = {
    set-env STARSHIP_STATUS $cmd-status-code
    set-env STARSHIP_DURATION (printf "%.0f" (* $edit:command-duration 1000))
    set-env STARSHIP_JOBS $num-bg-jobs
    ::STARSHIP:: prompt --right --logical-path=$pwd
}
