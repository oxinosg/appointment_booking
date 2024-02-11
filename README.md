# Appointment Booking

## Description
For simplicity, assume that the practice is always open from Monday to Friday
from 8:00 AM to 12:00 PM and from 1:00 PM to 5:00 PM. Below is the
representation of a practice's appointment calendar. The TODOs would be for
you to implement, in addition to a command-line interface for using the
application.

The core of the coding challenge is the function free_slots_optimized. This
function should filter the appointment offerings returned by the free_slots
function so that, if possible, slots are reserved for long-duration
appointment types while still allowing patients to book all types of
appointments as much as possible. It is assumed that it does not matter to a
patient exactly when within a 60-minute window the appointment is, as long as
it is within that window.

// TODO set up error system so that error codes will be used instead of strings.


## Run
```bash
cargo run
```

Then follow instructions.

## Tests
```bash
cargo test
```
