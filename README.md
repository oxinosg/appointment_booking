# Appointment Booking

## Description
Appointment booking system. The system is designed to allow patients to book
appointments with a practice. The practice offers appointments of three different types:
15 minutes, 30 minutes and 90 minutes.
The main function of the system is to provide a list of available appointment
slots for a given date range, while giving priority to long-duration appointments.

The booking system is designed with the following assumptions:
 - The practice is open from Monday to Friday from 8:00 AM to 12:00 PM and from 1:00 PM to 5:00 PM.
 - Time slots for appointments are always on the quarter hour (e.g. 8:00, 8:15, 8:30, 8:45, etc.).
 - Users do not care about the exact time within a 60-minute window, as long as it is within that window.

## Command-Line Interface
The system is designed to be used from the command line. The user can:
 - Add a new appointment. Any free time slot can be used. not only those provided by the optimized list.
 - See booked appointments
 - List available appointment slots for a given date range, for a given appointment type.
 - List available appointment slots for a given date range, for a given appointment type.
   Optimized to show a maximum of 1 appointment per 60 minutes. This optimizations is used
   to give as much space as possible to long-duration appointments.
 - Fill random appointments for a given appointment type, up to a given percentage.
 - Set the from and to date range for the previous commands to use. [default: now to end of the day]

## Optimization of appointments
The system is designed to give priority to long-duration appointments. This is done by using an optimization
algorithm that tries to maximize the number of long-duration appointments that can be booked.
When multiple time-slots for an appointment are available for the same 60-minute window, the system will check
which of those time-slots if used, will allow space for the most long-duration appointments to be booked in the future.

## TODOs
 - [ ] Set up error system so that error codes will be used instead of strings, and disallow unwrap usage (anyhow.rs).
 - [ ] Set up translation system for display names
 - [ ] Better validation for command-line arguments
 - [ ] Improve optimization for long-duration appointments by using historic data

## Run
```bash
cargo run
```

Then follow instructions.

## Tests
```bash
cargo test
```
