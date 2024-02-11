use std::collections::{BTreeMap, BTreeSet};

use chrono::{
    DateTime,
    Datelike,
    Duration,
    Local,
    NaiveDate,
    NaiveDateTime,
    NaiveTime,
    ParseError,
    Timelike,
};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use lazy_static::lazy_static;
use rand::Rng;

lazy_static! {
    // Static variable to hold the working hours. 8:00 to 12:00 and 13:00 to 17:00
    static ref WORKING_HOURS: [(NaiveTime, NaiveTime); 2] = [
        (
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        ),
        (
            NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
            NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
        ),
    ];

    // Static variable to hold the working days. Monday to Friday
    static ref WORKING_DAYS: [u32; 5] = [0, 1, 2, 3, 4];
}

// Trait to define the working day times utility functions
trait WorkingDayTimes {
    // Check if the current date is a working day
    fn is_working_day(&self) -> bool;

    // Check if the current time is within the working hours
    fn is_working_hour(&self) -> bool;

    // Check if the current date and time is within the working hours
    fn is_working_day_and_hour(&self) -> bool;

    // Get the next working date and time
    fn get_next_working_datetime(&self, appointment_type: Option<AppointmentType>)
        -> NaiveDateTime;

    // Function to append to `to` time the appointment duration
    fn calculate_end_time(self, appointment_type: AppointmentType) -> NaiveDateTime;
}

impl WorkingDayTimes for NaiveDateTime {
    /// Check if the current date is a working day
    fn is_working_day(&self) -> bool {
        WORKING_DAYS.contains(&self.date().weekday().num_days_from_monday())
    }

    /// Check if the current time is within the working hours
    fn is_working_hour(&self) -> bool {
        WORKING_HOURS
            .iter()
            .any(|(start, end)| self.time() >= *start && self.time() < *end)
    }

    /// Check if the current date and time is within the working hours
    fn is_working_day_and_hour(&self) -> bool {
        self.is_working_day() && self.is_working_hour()
    }

    /// Get the next working date and time
    ///
    /// This function gets the next 15 minute time slot that is within the
    /// working hours
    fn get_next_working_datetime(
        &self,
        appointment_type: Option<AppointmentType>,
    ) -> NaiveDateTime {
        // Get the current date and time
        let mut current = *self;

        // Round to the last 15 minute time
        current = current.date().and_time(
            NaiveTime::from_hms_opt(
                current.time().hour(),
                (current.time().minute() / 15) * 15,
                0,
            )
            .unwrap_or(current.time()),
        );

        // Get the time slot duration. If the appointment type is not provided, use the
        // default time slot duration of 15 minutes
        let time_slot_duration = if let Some(appointment_type) = appointment_type {
            appointment_type.duration()
        } else {
            // Default time slot duration is 15 minutes
            Duration::minutes(15)
        };

        // Append time slot duration to the current time
        current += time_slot_duration;

        if current.is_working_day_and_hour() {
            return current;
        } else {
            // Check if time is before the break
            if current.time() < WORKING_HOURS[0].0 {
                // Set the time to the start of the working hours
                current = current.date().and_time(WORKING_HOURS[0].0);
            } else if current.time() < WORKING_HOURS[1].0 {
                // Set the time to the start of the working hours
                current = current.date().and_time(WORKING_HOURS[1].0);
            } else {
                // Already past end of working day.
                // Set the time to the start of the next working hours
                current = current.date().and_time(WORKING_HOURS[0].0);

                // Get the next working day
                loop {
                    current += Duration::days(1);

                    if current.is_working_day() {
                        break;
                    }
                }
            }
        }

        current
    }

    /// Function to append to `to` time the appointment duration
    fn calculate_end_time(self, appointment_type: AppointmentType) -> NaiveDateTime {
        self + appointment_type.duration()
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct DoctorsAppointment {
    date_time: NaiveDateTime,
    appointment_type: AppointmentType,
}

impl DoctorsAppointment {
    /// Create a new doctor's appointment
    fn new(date_time: NaiveDateTime, appointment_type: AppointmentType) -> Self {
        Self {
            date_time,
            appointment_type,
        }
    }

    /// Convert the appointment into reserved time slots of 15 minutes
    fn to_reserved_time_slots(&self) -> Vec<NaiveDateTime> {
        let mut time_slots = vec![];

        let mut current = self.date_time;

        while current < self.date_time + self.appointment_type.duration() {
            time_slots.push(current);
            current += Duration::minutes(15);
        }

        time_slots
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum AppointmentType {
    DentalCheckUp,
    ImplantConsultation,
    UrgentDentalAppointment,
}

// Iterator to iterate through the appointment types
struct AppointmentTypeIter {
    next: Option<AppointmentType>,
}

impl AppointmentTypeIter {
    // Create a new appointment type iterator
    fn new() -> Self {
        AppointmentTypeIter {
            next: Some(AppointmentType::ImplantConsultation),
        }
    }
}

// Implement the iterator trait for the appointment type iterator
impl Iterator for AppointmentTypeIter {
    type Item = AppointmentType;

    // Iterate through the appointment types from high to low duration
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.next.take(); // Take the current value, leaving None in its place
        self.next = match result {
            Some(AppointmentType::ImplantConsultation) => Some(AppointmentType::DentalCheckUp),
            Some(AppointmentType::DentalCheckUp) => Some(AppointmentType::UrgentDentalAppointment),
            Some(AppointmentType::UrgentDentalAppointment) => None,
            None => None,
        };
        result
    }
}

impl AppointmentType {
    /// Get the duration of the appointment type
    fn duration(&self) -> Duration {
        match self {
            AppointmentType::DentalCheckUp => Duration::minutes(30),
            AppointmentType::ImplantConsultation => Duration::minutes(90),
            AppointmentType::UrgentDentalAppointment => Duration::minutes(15),
        }
    }

    /// Get the display name of the appointment type
    fn display_name(&self) -> &str {
        // TODO set up translation system for display names
        match self {
            AppointmentType::DentalCheckUp => "Check-up",
            AppointmentType::ImplantConsultation => "Implant Consultation",
            AppointmentType::UrgentDentalAppointment => "Urgent Appointment",
        }
    }

    /// Get the duration of the appointment type in 15 minute time slots
    fn duration_in_time_slots(&self) -> u8 {
        (self.duration().num_minutes() / 15) as u8
    }
}

// Define the doctor's calendar
struct DoctorsCalendar {
    appointments: BTreeSet<DoctorsAppointment>,
}

impl DoctorsCalendar {
    // Create a new doctor's calendar
    fn new() -> Self {
        Self {
            appointments: BTreeSet::new(),
        }
    }

    /// Add an appointment to the calendar
    fn add_appointment(&mut self, appointment: DoctorsAppointment) -> Result<(), String> {
        if appointment
            .to_reserved_time_slots()
            .iter()
            .any(|time_slot| !time_slot.is_working_day_and_hour())
        {
            return Err("Appointment is not within working hours".to_string());
        }

        // Get the list of existing appointments within the given time period
        let existing_appointments = self.booked_appointments(
            Some(appointment.date_time - appointment.appointment_type.duration()),
            Some(appointment.date_time),
        );

        // Check if the appointment overlaps with an existing appointment
        if existing_appointments.iter().any(|existing_appointment| {
            existing_appointment.date_time + existing_appointment.appointment_type.duration()
                > appointment.date_time
                && existing_appointment.date_time < appointment.date_time
        }) {
            return Err("Appointment overlaps with an existing appointment".to_string());
        }

        // Add the appointment to the calendar
        self.appointments.insert(appointment);

        Ok(())
    }

    /// Get the list of booked appointments
    fn booked_appointments(
        &self,
        from: Option<NaiveDateTime>,
        to: Option<NaiveDateTime>,
    ) -> Vec<DoctorsAppointment> {
        // Get the list of booked appointments
        let booked_appointments = self
            .appointments
            .iter()
            .filter(|appointment| {
                if let Some(from) = from {
                    if appointment.date_time < from {
                        return false;
                    }
                }

                if let Some(to) = to {
                    if appointment.date_time > to {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect::<Vec<DoctorsAppointment>>();

        booked_appointments
    }

    /// Fill the calendar with random appointments of the given type.
    ///
    /// The appointments will try to be filled up to the given percentage. If
    /// the percentage can't be reached because there are no more free slots for
    /// that appointment type, the function will stop.
    fn fill_random(
        &mut self,
        from: NaiveDateTime,
        to: NaiveDateTime,
        appointment_type: AppointmentType,
        filled_percentage: u8,
    ) {
        // Count the total time spots within the given time period
        let mut total_time_spots: u8 = 0;

        {
            let mut current = from;

            loop {
                current = current.get_next_working_datetime(None);

                if current > to {
                    break;
                }

                total_time_spots += 1;
            }
        }

        loop {
            // Get list of free slots for the given time period and appointment type
            let free_slots = self.free_slots(Some(from), Some(to), appointment_type);

            // If there are no free slots, break the loop
            if free_slots.is_empty() {
                break;
            }

            // Get random free slot from list to fill
            let random_index = rand::thread_rng().gen_range(0..free_slots.len());
            let random_slot = free_slots[random_index];

            // Create a new appointment for the random slot
            let appointment = DoctorsAppointment::new(random_slot, appointment_type);

            // Add the appointment to the calendar
            self.add_appointment(appointment).unwrap();

            // Get list of booked appointments
            let booked_appointments = self.booked_appointments(Some(from), Some(to));

            // Convert the booked appointments to reserved time slots
            let reserved_time_slots = booked_appointments
                .iter()
                .flat_map(|appointment| appointment.to_reserved_time_slots())
                .collect::<Vec<NaiveDateTime>>();

            // Check if the calendar is filled as much as possible up to the given
            // percentage
            if reserved_time_slots.len() as f64 / total_time_spots as f64 * 100.0
                == filled_percentage as f64
                || (reserved_time_slots.len() + appointment_type.duration_in_time_slots() as usize)
                    as f64
                    / total_time_spots as f64
                    * 100.0
                    > filled_percentage as f64
            {
                break;
            }
        }
    }

    /// Get list of available 15 minute time slots for the given time period
    fn available_single_time_slots(
        &self,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Vec<NaiveDateTime> {
        // Get list of existing appointments within the given time period
        let existing_appointments = self.booked_appointments(Some(from), Some(to));

        // Convert the booked appointments to reserved time slots
        let reserved_time_slots: Vec<NaiveDateTime> = existing_appointments
            .iter()
            .flat_map(|appointment| appointment.to_reserved_time_slots())
            .collect();

        // Create the list of available time slots
        let mut available_time_slots = vec![];

        // Start a loop from the `from` time to the `to` time
        let mut current = from;

        while current < to {
            // Check if the current time is within the working hours
            if current.is_working_day_and_hour() {
                // Check if the current time has already been reserved
                if !reserved_time_slots.contains(&current) {
                    // Add the current time to the list of available time slots
                    available_time_slots.push(current);
                }
            }

            // Increment the current time by 15 minutes
            current = current.get_next_working_datetime(None);
        }

        available_time_slots
    }

    /// Get the list of free time slots for the given time period and
    /// appointment type
    fn free_slots(
        &self,
        from: Option<NaiveDateTime>,
        to: Option<NaiveDateTime>,
        appointment_type: AppointmentType,
    ) -> Vec<NaiveDateTime> {
        // In case `from` is not provided, set it to the current time
        let from = if let Some(from) = from {
            from
        } else {
            now_next_15_mark()
        };

        // In case `to` is not provided, set it to the end of the day this Friday
        let to = if let Some(to) = to {
            to.calculate_end_time(appointment_type)
        } else {
            end_of_week()
        };

        // Get list of available time slots
        let available_time_slots = self.available_single_time_slots(from, to);

        // Filter the available time slots by the appointment type
        let filtered_time_slots = match appointment_type {
            // For urgent appointments, no need to filter the time slots
            AppointmentType::UrgentDentalAppointment => available_time_slots,
            AppointmentType::DentalCheckUp | AppointmentType::ImplantConsultation => {
                available_time_slots
                    .iter()
                    .filter(|time_slot| {
                        // Check if the following time slots are available for
                        // the appointment type to fit
                        let mut current = **time_slot;
                        let mut available = true;

                        for _ in 0..appointment_type.duration().num_minutes() / 15 {
                            if !available_time_slots.contains(&current) {
                                available = false;
                                break;
                            }

                            current += Duration::minutes(15);
                        }

                        available
                    })
                    .map(|time_slot| *time_slot)
                    .collect()
            },
        };

        filtered_time_slots
    }

    // A simple optimization is to keep at least one long-duration appointment until
    // X days are left

    /// Return the free slots, filtered to one appointment per 60 minute window.
    /// The priority goes to the long-duration appointments
    fn free_slots_optimized(
        &self,
        from: Option<NaiveDateTime>,
        to: Option<NaiveDateTime>,
        appointment_type: AppointmentType,
    ) -> Vec<NaiveDateTime> {
        // Get the list of free slots
        let free_slots = self.free_slots(from, to, appointment_type);

        // In case `from` is not provided, set it to the current time
        let from = if let Some(from) = from {
            from
        } else {
            now_next_15_mark()
        };

        // In case `to` is not provided, set it to the end of the day this Friday
        let to = if let Some(to) = to {
            to.calculate_end_time(appointment_type)
        } else {
            end_of_week()
        };

        let available_time_slots = self.available_single_time_slots(from, to);

        // Split into groups of 60 minutes windows
        let mut time_windows: BTreeMap<NaiveDateTime, Vec<&NaiveDateTime>> = BTreeMap::new();
        // Loop through the free slots and group them into 60 minute windows
        for slot in free_slots.iter() {
            // Get the start of the 60 minute window
            let window_start = slot.with_minute(0).unwrap().with_second(0).unwrap();

            // Insert window_start key in windows BTreeMap if it doesn't exist.
            // And add the slot as part of the array value
            time_windows
                .entry(window_start)
                .or_insert_with(Vec::new)
                .push(slot);
        }

        // The list of optimized free slots, one per 60 minute window
        let mut optimized_free_slots = vec![];

        // Loop through the windows and create a new list of connected free slots to
        // each value in the window
        for (_, window_time_slots) in time_windows.iter() {
            let mut ideal_slot: Option<&NaiveDateTime> = None;
            let mut ideal_count_appointment_space_per_type: BTreeMap<AppointmentType, u8> =
                BTreeMap::new();

            // Loop through the time slots in the time window and find the best slot
            for time_slot in window_time_slots.iter() {
                // Struct to hold the current count of appointment space per type
                let mut current_count_appointment_space_per_type: BTreeMap<AppointmentType, u8> =
                    BTreeMap::new();

                // Starts with 1 because the current slot is already connected
                let mut count_connected_single_slots_forwards = 0;
                let mut count_connected_single_slots_backwards = 0;

                // Find index of the current time slot in the available_time_slots
                let index = available_time_slots
                    .iter()
                    .position(|x| x == *time_slot)
                    .unwrap();

                // Move the index forward to check for connected slots excluding the current
                // appointment
                let mut index_forward = index + appointment_type.duration_in_time_slots() as usize;

                // Add all the next slots to the connected_slots if they are
                // available_time_slots
                loop {
                    // If next slot is not available, break
                    if available_time_slots.get(index_forward).is_none() {
                        break;
                    }

                    // If next slot is out of bounds
                    if available_time_slots.len() <= index_forward {
                        break;
                    }

                    // If next slot is directly connected increase the count of connected slots,
                    // otherwise break
                    if available_time_slots[index_forward] - available_time_slots[index_forward - 1]
                        == Duration::minutes(15)
                    {
                        count_connected_single_slots_forwards += 1;
                    } else {
                        break;
                    }

                    // Increment the index to check the next slot
                    index_forward += 1;
                }

                // Index for the current time slot while checking the connected previous slots
                let mut index_backward = index;
                // Add all the previous slots to the connected_slots if they are
                // available_time_slots
                loop {
                    // If previous slot is out of bounds, break
                    if index_backward == 0 {
                        break;
                    }

                    index_backward -= 1;

                    // If previous slot is not available, break
                    if available_time_slots.get(index_backward).is_none() {
                        break;
                    }

                    // If previous slot is directly connected increase the count of connected slots,
                    // otherwise break
                    if available_time_slots[index_backward + 1]
                        - available_time_slots[index_backward]
                        == Duration::minutes(15)
                    {
                        count_connected_single_slots_backwards += 1;
                    } else {
                        break;
                    }
                }

                // Count how many appointments fit in the connected_slots from
                // longer appointments to shorter
                let appointment_iter = AppointmentTypeIter::new();
                // Loop through the appointment types from longest to shortest
                for appointment_type in appointment_iter {
                    // Get the number of appointments that fit in the connected slots for the
                    // current appointment type
                    let num_appointments = count_connected_single_slots_forwards
                        / appointment_type.duration_in_time_slots()
                        + count_connected_single_slots_backwards
                            / appointment_type.duration_in_time_slots();

                    // Add the number of appointments to the current count of appointment space per
                    // type
                    current_count_appointment_space_per_type
                        .insert(appointment_type, num_appointments);

                    // Update connected count of slots, with the remainder of after removing the
                    // spots for the current appointment type
                    count_connected_single_slots_forwards = count_connected_single_slots_forwards
                        % appointment_type.duration_in_time_slots();
                    count_connected_single_slots_backwards = count_connected_single_slots_backwards
                        % appointment_type.duration_in_time_slots();
                }

                if let Some(_) = ideal_slot {
                    // Check if the current slot contains higher number of big appointment types
                    // than the current ideal slot, and if it does, set the current slot as the
                    // ideal one
                    let appointment_iter = AppointmentTypeIter::new();
                    for appointment_type in appointment_iter {
                        // If the current slot contains smaller number of big appointment types than
                        // the ideal slot, break
                        if current_count_appointment_space_per_type[&appointment_type]
                            < ideal_count_appointment_space_per_type[&appointment_type]
                        {
                            break;
                        }

                        // If the current slot contains higher number of big appointment types than
                        // the ideal slot, set the current slot as the ideal slot. Otherwise
                        // continue to the next/shorter appointment type
                        if current_count_appointment_space_per_type[&appointment_type]
                            > ideal_count_appointment_space_per_type[&appointment_type]
                        {
                            ideal_slot = Some(time_slot);
                            ideal_count_appointment_space_per_type =
                                current_count_appointment_space_per_type;
                            break;
                        }
                    }
                } else {
                    // If there is no ideal slot, set the current slot as the ideal slot
                    ideal_slot = Some(time_slot);
                    ideal_count_appointment_space_per_type =
                        current_count_appointment_space_per_type;
                }
            }

            // Add the ideal slot to the optimized free slots
            if let Some(ideal_slot) = ideal_slot {
                optimized_free_slots.push(*ideal_slot);
            }
        }

        optimized_free_slots
    }
}

/// Return a NaiveDateTime for the next 15 minute mark time from passed local
/// time
///
/// i.e. 18:12 => 18:15
fn next_15_mark(date: DateTime<Local>) -> NaiveDateTime {
    let mut minute = (date.minute() / 15) * 15 + 15;
    let mut hour = date.hour();

    if minute >= 60 {
        minute = 0;
        hour += 1;
    }

    // Get the next 15 minute mark time
    date.with_hour(hour)
        .unwrap_or_default()
        .with_minute(minute)
        .unwrap_or_default()
        .with_second(0)
        .unwrap_or_default()
        .with_nanosecond(0)
        .unwrap_or_default()
        .naive_utc()
}

/// Return a NaiveDateTime for the next 15 minute time from the current time
fn now_next_15_mark() -> NaiveDateTime {
    // Get the current time
    let now = Local::now();
    next_15_mark(now)
}

/// Return a NaiveDateTime for the end of the day
fn end_of_day() -> NaiveDateTime {
    // Get the current time
    let now = Local::now();

    // Get the end of the day
    now.with_hour(23)
        .unwrap()
        .with_minute(59)
        .unwrap()
        .with_second(59)
        .unwrap()
        .with_nanosecond(0)
        .unwrap()
        .naive_utc()
}


/// Return a NaiveDateTime for the end of the day this Friday
fn end_of_week() -> NaiveDateTime {
    // Get the current time
    let now = Local::now();

    // Get the end of the day
    let end_of_day = now.with_hour(23).unwrap().with_minute(59).unwrap();

    // Get the number of days until the end of the week
    let end_of_week = end_of_day.weekday().num_days_from_monday();

    // Get the NaiveDateTime for the end of the day this Friday
    let end_of_weekdays = end_of_day + Duration::days(7 - end_of_week as i64);
    end_of_weekdays.naive_utc()
}

/// Get a date from the user
fn get_date_from_user(prompt: &str) -> Result<NaiveDateTime, ParseError> {
    let date_str: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()
        .unwrap();

    NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M")
}

/// Enum that defines the actions that can be performed through the command-line
enum Action {
    SetFromDate,
    SetToDate,
    FillRandom,
    BookedAppointments,
    AddNewAppointment,
    ListFreeTimeSlots,
    ListOptimizedFreeTimeSlots,
    Quit,
}

fn main() {
    // Create a new doctor's calendar
    let mut calendar = DoctorsCalendar::new();

    // Set the default `from` and `to` dates
    let mut from = now_next_15_mark();
    let mut to = end_of_day();

    // Create a vector of tuples where each tuple contains the menu item string and
    // the associated enum variant
    let actions = vec![
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

    loop {
        println!("Current `from` date: {}", from);
        println!("Current `to` date: {}", to);
        println!("");


        // Display the menu and get the user's selection
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an action")
            .default(0)
            .items(&action_descriptions)
            .interact()
            .unwrap();

        match actions[selection].1 {
            Action::SetFromDate => {
                // TODO validate the from and to values:
                // - from is before to
                // - from is not in the past
                // - to is not in the past
                // - from is not too far in the future
                // - to is not too far in the future
                // - from and to have a cap of at least 60 minutes
                match get_date_from_user("Enter date (YYYY-MM-DD HH:MM) [default: start of today]")
                {
                    Ok(start_date) => {
                        from = start_date
                            .with_minute((start_date.minute() / 15) * 15)
                            .unwrap()
                            .with_second(0)
                            .unwrap()
                    },
                    Err(e) => println!("Failed to parse start date: {}", e),
                }
            },
            Action::SetToDate => {
                match get_date_from_user(
                    "Enter date (YYYY-MM-DD HH:MM) [default: end of the day today]",
                ) {
                    Ok(end_date) => {
                        to = end_date
                            .with_minute((end_date.minute() / 15) * 15)
                            .unwrap()
                            .with_second(0)
                            .unwrap()
                    },
                    Err(e) => println!("Failed to parse end date: {}", e),
                }
            },
            Action::FillRandom => {
                let actions = vec![
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
                let action_descriptions: Vec<&str> =
                    actions.iter().map(|(desc, _)| *desc).collect();

                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Choose an appointment type")
                    .default(0)
                    .items(&action_descriptions)
                    .interact()
                    .unwrap();

                let percentage: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter percentage to fill. [1 and 100]")
                    .validate_with(|input_str: &String| -> Result<(), &str> {
                        match input_str.parse::<i32>() {
                            Ok(num) if num >= 1 && num <= 100 => Ok(()),
                            _ => Err("Please enter a valid number between 1 and 100"),
                        }
                    })
                    .interact()
                    .unwrap();

                calendar.fill_random(
                    from,
                    to,
                    actions[selection].1,
                    percentage.parse::<u8>().unwrap(),
                );
            },
            Action::BookedAppointments => {
                let booked_appointments = calendar.booked_appointments(Some(from), Some(to));
                println!("Booked appointments: {:?}", booked_appointments);

                booked_appointments.iter().for_each(|appointment| {
                    println!(
                        "Date: {}, Type: {}",
                        appointment.date_time.to_string(),
                        appointment.appointment_type.display_name()
                    );
                });
            },
            Action::AddNewAppointment => {
                // Validate only 15 minute time slots are allowed

                // add new appointment
                let appointment = DoctorsAppointment::new(
                    NaiveDateTime::new(
                        NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                        NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                    ),
                    AppointmentType::DentalCheckUp,
                );

                let _result = calendar.add_appointment(appointment);
            },
            Action::ListFreeTimeSlots => {
                // List
                calendar.free_slots_optimized(Some(from), Some(to), AppointmentType::DentalCheckUp);
            },
            Action::ListOptimizedFreeTimeSlots => {
                calendar.free_slots_optimized(Some(from), Some(to), AppointmentType::DentalCheckUp);
            },
            Action::Quit => {
                println!("Exiting...");
                break;
            },
        }

        println!("");
    }
}


#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    // Test the is_working_day function
    fn test_is_working_day() {
        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        );
        assert!(date.is_working_day());

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 2).unwrap(),
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        );
        assert!(date.is_working_day());

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 3).unwrap(),
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        );
        assert!(!date.is_working_day());

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 4).unwrap(),
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        );
        assert!(!date.is_working_day());

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 5).unwrap(),
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        );
        assert!(date.is_working_day());

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 6).unwrap(),
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        );
        assert!(date.is_working_day());

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 7).unwrap(),
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        );
        assert!(date.is_working_day());
    }

    #[test]
    // Test the is_working_hour method
    fn test_is_working_hour() {
        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(7, 59, 59).unwrap(),
        );
        assert!(!date.is_working_hour());

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        );
        assert!(date.is_working_hour());

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(11, 59, 59).unwrap(),
        );
        assert!(date.is_working_hour());

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        );
        assert!(!date.is_working_hour());

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(12, 59, 59).unwrap(),
        );
        assert!(!date.is_working_hour());

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
        );
        assert!(date.is_working_hour());

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(16, 59, 59).unwrap(),
        );
        assert!(date.is_working_hour());

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(17, 00, 00).unwrap(),
        );
        assert!(!date.is_working_hour());
    }

    // Test get next working datetime function
    #[test]
    fn test_get_next_working_datetime() {
        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(7, 45, 00).unwrap(),
        );
        let next = date.get_next_working_datetime(None);
        assert_eq!(
            next,
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(8, 0, 0).unwrap()
            )
        );

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        );
        let next = date.get_next_working_datetime(None);
        assert_eq!(
            next,
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(13, 0, 0).unwrap()
            )
        );

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
        );
        let next = date.get_next_working_datetime(None);
        assert_eq!(
            next,
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 2).unwrap(),
                NaiveTime::from_hms_opt(8, 0, 0).unwrap()
            )
        );
    }

    // Test the calculate_end_time function
    #[test]
    fn test_calculate_end_time() {
        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        );
        let end_time = date.calculate_end_time(AppointmentType::DentalCheckUp);
        assert_eq!(
            end_time,
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(8, 30, 0).unwrap()
            )
        );

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        );
        let end_time = date.calculate_end_time(AppointmentType::ImplantConsultation);
        assert_eq!(
            end_time,
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(9, 30, 0).unwrap()
            )
        );

        let date = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        );
        let end_time = date.calculate_end_time(AppointmentType::UrgentDentalAppointment);
        assert_eq!(
            end_time,
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(8, 15, 0).unwrap()
            )
        );
    }

    // Test the to_reserved_time_slots function
    #[test]
    fn test_to_reserved_time_slots() {
        let appointment = DoctorsAppointment::new(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            ),
            AppointmentType::DentalCheckUp,
        );
        let reserved_time_slots = appointment.to_reserved_time_slots();
        assert_eq!(reserved_time_slots.len(), 2);
        assert_eq!(
            reserved_time_slots[0],
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(8, 0, 0).unwrap()
            )
        );
        assert_eq!(
            reserved_time_slots[1],
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(8, 15, 0).unwrap()
            )
        );

        let appointment = DoctorsAppointment::new(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            ),
            AppointmentType::ImplantConsultation,
        );

        let reserved_time_slots = appointment.to_reserved_time_slots();
        assert_eq!(reserved_time_slots.len(), 6);
        assert_eq!(
            reserved_time_slots[0],
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(8, 0, 0).unwrap()
            )
        );
        assert_eq!(
            reserved_time_slots[1],
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(8, 15, 0).unwrap()
            )
        );
        assert_eq!(
            reserved_time_slots[2],
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(8, 30, 0).unwrap()
            )
        );
    }

    // Test the creation of a new DoctorsCalendar
    #[test]
    fn test_new_doctors_calendar() {
        let calendar = DoctorsCalendar::new();
        assert!(calendar.appointments.is_empty());
    }

    // Test the add_appointment function
    #[test]
    fn test_add_appointment() {
        let mut calendar = DoctorsCalendar::new();

        let appointment = DoctorsAppointment::new(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            ),
            AppointmentType::DentalCheckUp,
        );

        let result = calendar.add_appointment(appointment);
        assert!(result.is_ok());
        assert_eq!(calendar.appointments.len(), 1);

        let appointment = DoctorsAppointment::new(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(8, 15, 0).unwrap(),
            ),
            AppointmentType::DentalCheckUp,
        );

        let result = calendar.add_appointment(appointment);
        assert!(result.is_err());
        assert_eq!(calendar.appointments.len(), 1);
    }

    // Test the booked_appointments function
    #[test]
    fn test_booked_appointments() {
        let mut calendar = DoctorsCalendar::new();

        let appointment = DoctorsAppointment::new(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            ),
            AppointmentType::DentalCheckUp,
        );

        let _ = calendar.add_appointment(appointment);

        let booked_appointments = calendar.booked_appointments(None, None);
        assert_eq!(booked_appointments.len(), 1);
        assert_eq!(booked_appointments[0].date_time, appointment.date_time);

        let booked_appointments = calendar.booked_appointments(
            Some(appointment.date_time - Duration::minutes(15)),
            Some(appointment.date_time + Duration::minutes(15)),
        );
        assert_eq!(booked_appointments.len(), 1);
        assert_eq!(booked_appointments[0].date_time, appointment.date_time);

        let booked_appointments = calendar.booked_appointments(
            Some(appointment.date_time + Duration::minutes(15)),
            Some(appointment.date_time + Duration::minutes(45)),
        );
        assert_eq!(booked_appointments.len(), 0);

        // Add more appointments
        let appointment = DoctorsAppointment::new(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
            ),
            AppointmentType::DentalCheckUp,
        );

        let _ = calendar.add_appointment(appointment);

        let appointment = DoctorsAppointment::new(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
            ),
            AppointmentType::DentalCheckUp,
        );

        let _ = calendar.add_appointment(appointment);

        let appointment = DoctorsAppointment::new(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 2).unwrap(),
                NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            ),
            AppointmentType::DentalCheckUp,
        );

        let _ = calendar.add_appointment(appointment);

        // Get booked appointments for the whole day
        let booked_appointments = calendar.booked_appointments(
            Some(NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            )),
            Some(NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
            )),
        );
        assert_eq!(booked_appointments.len(), 3);

        // Get booked appointments for the whole day and turn them into reserved time
        // slots
        let reserved_time_slots = booked_appointments
            .iter()
            .flat_map(|appointment| appointment.to_reserved_time_slots())
            .collect::<Vec<NaiveDateTime>>();

        // Turn the reserved time slots into a stings of the format "YYYY-MM-DD
        // HH:MM:SS"
        let reserved_time_slots = reserved_time_slots
            .iter()
            .map(|time_slot| time_slot.to_string())
            .collect::<Vec<String>>();

        assert_eq!(reserved_time_slots.len(), 6);
    }

    // Test available_time_slots
    #[test]
    fn test_available_time_slots() {
        let mut calendar = DoctorsCalendar::new();

        let from = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        );

        let to = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
        );

        let available_time_slots = calendar.available_single_time_slots(from, to);
        assert_eq!(available_time_slots.len(), 32);

        let from = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        );

        let to = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
        );

        let available_time_slots = calendar.available_single_time_slots(from, to);
        assert_eq!(available_time_slots.len(), 16);

        // Test multiple days
        let from = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        );

        let to = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 2).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
        );

        let available_time_slots = calendar.available_single_time_slots(from, to);
        assert_eq!(available_time_slots.len(), 64);

        // Add many appointments first then check what is available
        let appointment = DoctorsAppointment::new(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            ),
            AppointmentType::DentalCheckUp,
        );
        calendar.add_appointment(appointment).unwrap();

        let appointment = DoctorsAppointment::new(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(8, 30, 0).unwrap(),
            ),
            AppointmentType::DentalCheckUp,
        );
        calendar.add_appointment(appointment).unwrap();

        // Different appointment type
        let appointment = DoctorsAppointment::new(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            ),
            AppointmentType::ImplantConsultation,
        );
        calendar.add_appointment(appointment).unwrap();

        let appointment = DoctorsAppointment::new(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(11, 30, 0).unwrap(),
            ),
            AppointmentType::ImplantConsultation,
        );

        // Appointment not allowed to be added. Middle of break
        assert!(calendar.add_appointment(appointment).is_err());

        // Long appointment type
        let appointment = DoctorsAppointment::new(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
            ),
            AppointmentType::ImplantConsultation,
        );
        calendar.add_appointment(appointment).unwrap();

        // Short appointment type
        let appointment = DoctorsAppointment::new(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(15, 0, 0).unwrap(),
            ),
            AppointmentType::UrgentDentalAppointment,
        );
        calendar.add_appointment(appointment).unwrap();


        let from = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        );

        let to = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
        );

        let available_time_slots = calendar.available_single_time_slots(from, to);
        let available_time_slots = available_time_slots
            .iter()
            .map(|time_slot| time_slot.to_string())
            .collect::<Vec<String>>();
        assert_eq!(available_time_slots.len(), 15);

        // Test the free_slots function
        let from = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        );
        let to = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
        );

        let free_slots_dental =
            calendar.free_slots(Some(from), Some(to), AppointmentType::DentalCheckUp);
        let free_slots_dental = free_slots_dental
            .iter()
            .map(|time_slot| time_slot.to_string())
            .collect::<Vec<String>>();
        assert_eq!(free_slots_dental.len(), 11);

        let free_slots_implant =
            calendar.free_slots(Some(from), Some(to), AppointmentType::ImplantConsultation);
        let free_slots_implant = free_slots_implant
            .iter()
            .map(|time_slot| time_slot.to_string())
            .collect::<Vec<String>>();
        assert_eq!(free_slots_implant.len(), 2);

        let free_slots_urg = calendar.free_slots(
            Some(from),
            Some(to),
            AppointmentType::UrgentDentalAppointment,
        );
        let free_slots_urg = free_slots_urg
            .iter()
            .map(|time_slot| time_slot.to_string())
            .collect::<Vec<String>>();
        assert_eq!(free_slots_urg.len(), 15);
    }

    // Test the free_slots function
    #[test]
    fn test_free_slots() {
        let calendar = DoctorsCalendar::new();

        let from = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        );

        let to = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
        );

        let free_slots_urg = calendar.free_slots(
            Some(from),
            Some(to),
            AppointmentType::UrgentDentalAppointment,
        );
        let free_slots_urg = free_slots_urg
            .iter()
            .map(|time_slot| time_slot.to_string())
            .collect::<Vec<String>>();
        assert_eq!(free_slots_urg.len(), 32);

        // Test for multiple days
        let from = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        );

        let to = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 2).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
        );

        let free_slots_urg = calendar.free_slots(
            Some(from),
            Some(to),
            AppointmentType::UrgentDentalAppointment,
        );
        let free_slots_urg = free_slots_urg
            .iter()
            .map(|time_slot| time_slot.to_string())
            .collect::<Vec<String>>();
        assert_eq!(free_slots_urg.len(), 64);

        // Whole week
        let from = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        );

        let to = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 7).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
        );

        let free_slots_urg = calendar.free_slots(
            Some(from),
            Some(to),
            AppointmentType::UrgentDentalAppointment,
        );
        let free_slots_urg = free_slots_urg
            .iter()
            .map(|time_slot| time_slot.to_string())
            .collect::<Vec<String>>();
        assert_eq!(free_slots_urg.len(), 160);
    }

    // Test the free_slots_optimized function
    #[test]
    fn test_free_slots_optimized() {
        let mut calendar = DoctorsCalendar::new();

        let appointment = DoctorsAppointment::new(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            ),
            AppointmentType::DentalCheckUp,
        );
        calendar.add_appointment(appointment).unwrap();

        let appointment = DoctorsAppointment::new(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(11, 30, 0).unwrap(),
            ),
            AppointmentType::DentalCheckUp,
        );
        calendar.add_appointment(appointment).unwrap();

        let appointment = DoctorsAppointment::new(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
            ),
            AppointmentType::UrgentDentalAppointment,
        );
        calendar.add_appointment(appointment).unwrap();

        let appointment = DoctorsAppointment::new(
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
                NaiveTime::from_hms_opt(15, 45, 0).unwrap(),
            ),
            AppointmentType::DentalCheckUp,
        );
        calendar.add_appointment(appointment).unwrap();

        let from = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        );

        let to = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
        );

        let free_slots_optimized = calendar.free_slots_optimized(
            Some(from),
            Some(to),
            AppointmentType::ImplantConsultation,
        );
        let free_slots_optimized = free_slots_optimized
            .iter()
            .map(|time_slot| time_slot.to_string())
            .collect::<Vec<String>>();

        assert_eq!(free_slots_optimized.len(), 4);

        let free_slots_optimized =
            calendar.free_slots_optimized(Some(from), Some(to), AppointmentType::DentalCheckUp);
        let free_slots_optimized = free_slots_optimized
            .iter()
            .map(|time_slot| time_slot.to_string())
            .collect::<Vec<String>>();

        assert_eq!(free_slots_optimized.len(), 8);

        let free_slots_optimized = calendar.free_slots_optimized(
            Some(from),
            Some(to),
            AppointmentType::UrgentDentalAppointment,
        );
        let free_slots_optimized = free_slots_optimized
            .iter()
            .map(|time_slot| time_slot.to_string())
            .collect::<Vec<String>>();

        assert_eq!(free_slots_optimized.len(), 8);
    }

    // Test fill_random function
    #[test]
    fn test_fill_random() {
        let mut calendar = DoctorsCalendar::new();

        let from = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        );

        let to = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
        );

        calendar.fill_random(from, to, AppointmentType::UrgentDentalAppointment, 90);

        // Get list of booked appointments
        let booked_appointments = calendar.booked_appointments(Some(from), Some(to));

        assert_eq!(booked_appointments.len(), 28);

        let mut calendar = DoctorsCalendar::new();

        calendar.fill_random(from, to, AppointmentType::UrgentDentalAppointment, 100);

        let booked_appointments = calendar.booked_appointments(Some(from), Some(to));

        assert_eq!(booked_appointments.len(), 32);


        let mut calendar = DoctorsCalendar::new();

        calendar.fill_random(from, to, AppointmentType::DentalCheckUp, 20);

        let booked_appointments = calendar.booked_appointments(Some(from), Some(to));

        assert_eq!(booked_appointments.len(), 3);

        let from = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
            NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        );

        let to = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 2, 2).unwrap(),
            NaiveTime::from_hms_opt(23, 59, 59).unwrap(),
        );

        let mut calendar = DoctorsCalendar::new();

        calendar.fill_random(from, to, AppointmentType::DentalCheckUp, 40);

        let booked_appointments = calendar.booked_appointments(Some(from), Some(to));

        assert_eq!(booked_appointments.len(), 12);
    }
}
