/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_print_reverse_alphabet.rs                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: welee <welee@student.42singapore.sg>       +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/15 10:43:55 by welee             #+#    #+#             */
/*   Updated: 2025/05/26 16:39:49 by welee            ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io::Write;
use crate::ft_print_iter;

/// Prints the lowercase alphabet in reverse order to the provided writer.
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
