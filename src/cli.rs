//! This module contains the command-line interface (CLI) functions for the
//! application.

use chrono::{NaiveDateTime, ParseError, Timelike};
use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::appointment::AppointmentType;


/// Get appointment type from the user
pub fn get_appointment_type_from_user() -> AppointmentType {
    let actions_appointment_type = [
        (
            AppointmentType::UrgentDentalAppointment.display_name(),
            AppointmentType::UrgentDentalAppointment,
        ),
        (
            AppointmentType::DentalCheckUp.display_name(),
            AppointmentType::DentalCheckUp,
        ),
        (
            AppointmentType::ImplantConsultation.display_name(),
            AppointmentType::ImplantConsultation,
        ),
    ];

    // Extract the string descriptions to display in the menu
    let action_descriptions_appointment_type: Vec<&str> = actions_appointment_type
        .iter()
        .map(|(desc, _)| *desc)
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an appointment type")
        .default(0)
        .items(&action_descriptions_appointment_type)
        .interact()
        .unwrap();

    actions_appointment_type[selection].1
}


/// Get a date from the user
pub fn get_date_from_user(prompt: &str) -> Result<NaiveDateTime, ParseError> {
    let date_str: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()
        .unwrap();

    NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M")
}

/// Enum that defines the actions that can be performed through the command-line
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    SetFromDate,
    SetToDate,
    FillRandom,
    BookedAppointments,
    AddNewAppointment,
    ListFreeTimeSlots,
    ListOptimizedFreeTimeSlots,
    Quit,
}

/// Display the main menu and return the user's selection
pub fn main_menu() -> Action {
    // Create a vector of tuples where each tuple contains the menu item string and
    // the associated enum variant
    let actions = [
        ("Add new appointment", Action::AddNewAppointment),
        ("Booked appointments", Action::BookedAppointments),
        ("List free time slots", Action::ListFreeTimeSlots),
        (
            "List optimized free time slots",
            Action::ListOptimizedFreeTimeSlots,
        ),
        ("Fill random", Action::FillRandom),
        ("Set `From` date", Action::SetFromDate),
        ("Set `To` date", Action::SetToDate),
        ("Quit", Action::Quit),
    ];

    // Extract the string descriptions to display in the menu
    let action_descriptions: Vec<&str> = actions.iter().map(|(desc, _)| *desc).collect();


    // Display the menu and get the user's selection
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an action")
        .default(0)
        .items(&action_descriptions)
        .interact()
        .unwrap();

    // Return the selected enum action
    actions[selection].1
}

/// Display the SetFromDate menu and return the user's selection
pub fn set_from_date_menu() -> Option<NaiveDateTime> {
    match get_date_from_user("Enter date (YYYY-MM-DD HH:MM) [default: start of today]") {
        Ok(start_date) => {
            return Some(
                start_date
                    .with_minute((start_date.minute() / 15) * 15)
                    .unwrap()
                    .with_second(0)
                    .unwrap(),
            )
        },
        Err(e) => println!("Failed to parse start date: {}", e),
    }

    None
}

/// Display the ToFromDate menu and return the user's selection
pub fn set_to_date_menu() -> Option<NaiveDateTime> {
    match get_date_from_user("Enter date (YYYY-MM-DD HH:MM) [default: end of the day today]") {
        Ok(end_date) => {
            return Some(
                end_date
                    .with_minute((end_date.minute() / 15) * 15)
                    .unwrap()
                    .with_second(0)
                    .unwrap(),
            )
        },
        Err(e) => println!("Failed to parse end date: {}", e),
    }

    None
}

/// Display the FillRandom menu and return the user's selection
pub fn fill_random_menu() -> (AppointmentType, u8) {
    let appointment_type = get_appointment_type_from_user();

    let percentage: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter percentage to fill. [1 and 100]")
        .validate_with(|input_str: &String| -> Result<(), &str> {
            match input_str.parse::<i32>() {
                Ok(num) if (1..=100).contains(&num) => Ok(()),
                _ => Err("Please enter a valid number between 1 and 100"),
            }
        })
        .interact()
        .unwrap();

    (appointment_type, percentage.parse::<u8>().unwrap())
}

/// Display the AddNewAppointment menu and return the user's selection
pub fn add_new_appointment_menu() -> (AppointmentType, Option<NaiveDateTime>) {
    let appointment_type = get_appointment_type_from_user();

    let date = match get_date_from_user("Enter date (YYYY-MM-DD HH:MM) [default: start of today]") {
        Ok(date) => {
            let date = date
                .with_minute((date.minute() / 15) * 15)
                .unwrap()
                .with_second(0)
                .unwrap();

            Some(date)
        },
        Err(e) => {
            println!("Failed to parse start date: {}", e);
            None
        },
    };

    (appointment_type, date)
}
