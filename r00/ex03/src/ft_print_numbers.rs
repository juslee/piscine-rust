/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_print_numbers.rs                                :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: welee <welee@student.42singapore.sg>       +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/15 10:43:55 by welee             #+#    #+#             */
/*   Updated: 2025/05/26 16:49:30 by welee            ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io::Write;
use crate::ft_print_iter;

/// Prints the numbers from 0 to 9 to the given writer.
pub fn ft_print_numbers<W: Write>(mut w: W) {
	ft_print_iter::ft_print_iter(&mut w,
		(0..=9)
		.map(|digit| (b'0' + digit as u8) as char)
		.collect::<Vec<char>>()
		.into_iter()
	);
}

#[macro_export]
macro_rules! ft_print_numbers {
	() => {
		ft_print_numbers::ft_print_numbers(std::io::stdout())
	};
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_ft_print_numbers_buffer() {
		let mut buf = Vec::new();
		ft_print_numbers(&mut buf);
		assert_eq!(buf, b"0123456789");
	}

	#[test]
	fn test_ft_print_numbers_unicode() {
		let mut buf = Vec::new();
		ft_print_numbers(&mut buf);
		assert_eq!(std::str::from_utf8(&buf).unwrap(), "0123456789");
	}
}
