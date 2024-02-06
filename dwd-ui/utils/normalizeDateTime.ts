import dayjs from "dayjs";
import "dayjs/plugin/customParseFormat";

/**
 * Normalizes a date and time string to a standardized format.
 *
 * @param date_time_str - The input date and time string to normalize.
 * @param start - Optional. Determines whether to set the omitted values to the start or the end. Defaults to true.
 * @returns The normalized date and time string in the format 'YYYY-MM-DD HH:MM', or undefined if the input string is invalid.
 *
 * @example
 * normalizeDateTime('2022', true); // Returns '2022-01-01 00:00'
 * normalizeDateTime('2022-11-10', true); // Returns '2022-11-10 00:00'
 * normalizeDateTime('2021-01', false); // Returns '2021-01-31 23:59'
 */
export function normalizeDateTime(
  date_time_str: string,
  start = true,
): string | undefined {
  //expected Format: YYYY-MM-DD hh:mm
  date_time_str = date_time_str.trim();
  const regex =
    /(?<year>\d{4})(-(?<month>\d{1,2})(-(?<day>\d{1,2})(( |T)(?<hour>\d{2})(:(?<minute>\d{2}))?)?)?)?/;
  const res = regex.exec(date_time_str);
  if (!res) {
    return; //TODO: throw error
  }

  const year = res?.groups?.year;
  let month = res?.groups?.month;
  let day = res?.groups?.day;
  let hour = res?.groups?.hour;
  let minute = res?.groups?.minute;

  if (!month) {
    if (start) {
      month = "01";
    } else {
      month = "12";
    }
  } else if (month.length === 1) {
    month = "0" + month;
  }

  if (!day) {
    if (start) {
      day = "01";
    } else {
      day = dayjs(`${year}-${month}`).daysInMonth().toString();
    }
  } else if (day.length === 1) {
    day = "0" + day;
  }

  if (!hour) {
    if (start) {
      hour = "00";
    } else {
      hour = "23";
    }
  }

  if (!minute) {
    if (start) {
      minute = "00";
    } else {
      minute = "59";
    }
  }

  const date_time = dayjs(
    `${year}-${month}-${day} ${hour}:${minute}`,
    "YYYY-MM-DD HH:mm",
    true,
  );
  if (!date_time.isValid()) {
    return; //TODO: throw error
  }
  return date_time.format("YYYY-MM-DDTHH:mm");
}
