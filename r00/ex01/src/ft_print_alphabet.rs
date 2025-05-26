/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_print_alphabet.rs                               :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: welee <welee@student.42singapore.sg>       +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/15 10:43:55 by welee             #+#    #+#             */
/*   Updated: 2025/05/26 15:02:37 by welee            ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io::Write;
use crate::ft_putchar::ft_putchar;

/// Prints the lowercase alphabet from 'a' to 'z' to the standard output.
/// This function uses the `ft_putchar` function to print each character.
/// It iterates over the ASCII values of lowercase letters and converts them to characters.
/// The `ft_putchar` function is called for each character to print it.
/// The function does not return any value.
/// It is designed to be used with any type that implements the `Write` trait,
pub fn ft_print_alphabet<W: Write>(mut w: W) {
	for c in b'a'..=b'z' {
		ft_putchar(&mut w, c as char);
	}
}

#[macro_export]
macro_rules! ft_print_alphabet {
	() => {
		ft_print_alphabet::ft_print_alphabet(std::io::stdout())
	};
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_ft_print_alphabet_buffer() {
		let mut buf = Vec::new();
		ft_print_alphabet(&mut buf);
		assert_eq!(buf, b"abcdefghijklmnopqrstuvwxyz");
	}

	#[test]
	fn test_ft_print_alphabet_unicode() {
		let mut buf = Vec::new();
		ft_print_alphabet(&mut buf);
		assert_eq!(std::str::from_utf8(&buf).unwrap(), "abcdefghijklmnopqrstuvwxyz");
	}
}
