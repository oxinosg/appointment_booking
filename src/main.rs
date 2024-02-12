use appointment_booking::{appointment::*, cli::*, utils::*};

fn main() {
    // Create a new doctor's calendar
    let mut calendar = DoctorsCalendar::new();

    // Set the default `from` and `to` dates
    let mut from = now_next_15_mark();
    let mut to = end_of_day();

    loop {
        println!("Current `from` date: {}", from);
        println!("Current `to` date: {}", to);
        println!();

        match main_menu() {
            Action::SetFromDate => {
                // Display the menu and get date from user
                if let Some(date) = set_from_date_menu() {
                    from = date;
                }
            },
            Action::SetToDate => {
                // Display the menu and get date from user
                if let Some(date) = set_to_date_menu() {
                    to = date;
                }
            },
            Action::FillRandom => {
                // Display the menu and get appointment type and percentage from user
                let (appointment_type, percentage) = fill_random_menu();

                // Fill the calendar with random appointments
                calendar.fill_random(from, to, appointment_type, percentage);
            },
            Action::BookedAppointments => {
                // Get booked appointments
                let booked_appointments = calendar.booked_appointments(Some(from), Some(to));

                // List all booked appointments
                booked_appointments.iter().for_each(|appointment| {
                    println!(
                        "Date: {}, Type: {}",
                        appointment.date_time,
                        appointment.appointment_type.display_name()
                    );
                });
            },
            Action::AddNewAppointment => {
                // Display the menu and get appointment type and date from user
                let (appointment_type, date) = add_new_appointment_menu();

                if let Some(date) = date {
                    // Create new appointment
                    let appointment = DoctorsAppointment::new(date, appointment_type);

                    // Add the appointment to the calendar
                    let result = calendar.add_appointment(appointment);

                    // Handle the result
                    if let Err(e) = result {
                        println!("Failed to add appointment: {}", e);
                    } else {
                        println!("Appointment added successfully");
                    }
                }
            },
            Action::ListFreeTimeSlots => {
                // Display the menu and get appointment type from user
                let appointment_type = get_appointment_type_from_user();

                // Get free time slots
                let slots = calendar.free_slots(Some(from), Some(to), appointment_type);

                // Display time slots
                println!("Free time slots:");
                slots.iter().for_each(|slot| println!("{}", slot));
            },
            Action::ListOptimizedFreeTimeSlots => {
                // Display the menu and get appointment type from user
                let appointment_type = get_appointment_type_from_user();

                // Get free optimized time slots
                let slots = calendar.free_slots_optimized(Some(from), Some(to), appointment_type);

                // Display optimized time slots
                println!("Optimized free time slots:");
                slots.iter().for_each(|slot| println!("{}", slot));
            },
            Action::Quit => {
                println!("Exiting...");
                break;
            },
        }

        println!();
        println!("====================================");
    }
}
