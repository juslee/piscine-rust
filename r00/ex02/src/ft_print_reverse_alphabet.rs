/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_print_reverse_alphabet.rs                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: welee <welee@student.42singapore.sg>       +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/15 10:43:55 by welee             #+#    #+#             */
/*   Updated: 2025/05/26 15:57:50 by welee            ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io::Write;
use crate::ft_print_iter;

/// Prints the lowercase alphabet from 'z' to 'a' to the standard output.
/// This function uses the `ft_putchar` function to print each character.
/// It iterates over the ASCII values of lowercase letters and converts them to characters.
/// The `ft_putchar` function is called for each character to print it.
/// The function does not return any value.
/// It is designed to be used with any type that implements the `Write` trait,
pub fn ft_print_reverse_alphabet<W: Write>(mut w: W) {
	ft_print_iter::ft_print_iter(&mut w, (b'a'..=b'z').rev());
}

#[macro_export]
macro_rules! ft_print_reverse_alphabet {
	() => {
		ft_print_reverse_alphabet::ft_print_reverse_alphabet(std::io::stdout())
	};
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_ft_print_reverse_alphabet_buffer() {
		let mut buf = Vec::new();
		ft_print_reverse_alphabet(&mut buf);
		assert_eq!(buf, b"zyxwvutsrqponmlkjihgfedcba");
	}

	#[test]
	fn test_ft_print_reverse_alphabet_unicode() {
		let mut buf = Vec::new();
		ft_print_reverse_alphabet(&mut buf);
		assert_eq!(std::str::from_utf8(&buf).unwrap(), "zyxwvutsrqponmlkjihgfedcba");
	}
}
