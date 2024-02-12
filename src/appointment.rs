//! Main file for the appointment system

use std::collections::{BTreeMap, BTreeSet};

use chrono::{Datelike, Duration, NaiveDateTime, NaiveTime, Timelike};
use lazy_static::lazy_static;
use rand::Rng;

use crate::utils::{end_of_week, now_next_15_mark};

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
pub trait WorkingDayTimes {
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
        }

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

        current
    }

    /// Function to append to `to` time the appointment duration
    fn calculate_end_time(self, appointment_type: AppointmentType) -> NaiveDateTime {
        self + appointment_type.duration()
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct DoctorsAppointment {
    pub date_time: NaiveDateTime,
    pub appointment_type: AppointmentType,
}

impl DoctorsAppointment {
    /// Create a new doctor's appointment
    pub fn new(date_time: NaiveDateTime, appointment_type: AppointmentType) -> Self {
        Self {
            date_time,
            appointment_type,
        }
    }

    /// Convert the appointment into reserved time slots of 15 minutes
    pub fn to_reserved_time_slots(self) -> Vec<NaiveDateTime> {
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
pub enum AppointmentType {
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
    pub fn duration(&self) -> Duration {
        match self {
            AppointmentType::DentalCheckUp => Duration::minutes(30),
            AppointmentType::ImplantConsultation => Duration::minutes(90),
            AppointmentType::UrgentDentalAppointment => Duration::minutes(15),
        }
    }

    /// Get the display name of the appointment type
    pub fn display_name(&self) -> &str {
        match self {
            AppointmentType::DentalCheckUp => "Check-up",
            AppointmentType::ImplantConsultation => "Implant Consultation",
            AppointmentType::UrgentDentalAppointment => "Urgent Appointment",
        }
    }

    /// Get the duration of the appointment type in 15 minute time slots
    pub fn duration_in_time_slots(&self) -> u8 {
        (self.duration().num_minutes() / 15) as u8
    }
}

// Define the doctor's calendar
pub struct DoctorsCalendar {
    pub appointments: BTreeSet<DoctorsAppointment>,
}

impl Default for DoctorsCalendar {
    fn default() -> Self {
        Self::new()
    }
}

impl DoctorsCalendar {
    // Create a new doctor's calendar
    pub fn new() -> Self {
        Self {
            appointments: BTreeSet::new(),
        }
    }

    /// Add an appointment to the calendar
    pub fn add_appointment(&mut self, appointment: DoctorsAppointment) -> Result<(), String> {
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
    pub fn booked_appointments(
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
    /// Existing appointments will be counted towards the percentage.
    pub fn fill_random(
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
    pub fn available_single_time_slots(
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
    pub fn free_slots(
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
                    .copied()
                    .collect()
            },
        };

        filtered_time_slots
    }

    /// Return the free slots, filtered to one appointment per 60 minute window.
    /// The priority goes to the long-duration appointments
    pub fn free_slots_optimized(
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
            time_windows.entry(window_start).or_default().push(slot);
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
                    count_connected_single_slots_forwards %=
                        appointment_type.duration_in_time_slots();
                    count_connected_single_slots_backwards %=
                        appointment_type.duration_in_time_slots();
                }

                if ideal_slot.is_some() {
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
