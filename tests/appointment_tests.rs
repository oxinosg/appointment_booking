//! Tests for the appointment module.
use appointment_booking::appointment::*;

#[cfg(test)]
mod tests {
    use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};

    use super::*;

    #[test]
    // Test the is_working_day function
    fn test_is_working_day() {
        let from_ymd_opt = NaiveDate::from_ymd_opt(2024, 2, 1);
        let date = NaiveDateTime::new(
            from_ymd_opt.unwrap(),
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

        let mut calendar = DoctorsCalendar::new();

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
