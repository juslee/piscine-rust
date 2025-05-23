/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_putchar.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: welee <welee@student.42singapore.sg>       +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/15 10:43:55 by welee             #+#    #+#             */
/*   Updated: 2025/05/23 17:07:56 by welee            ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io::Write;

/// Prints a single character to the standard output.
/// This function takes a character as input and writes it to the standard output.
/// It uses the `io::stdout()` function to get a handle to the standard output stream.
/// The character is written using the `write!` macro, which formats the output.
/// Finally, it flushes the output buffer to ensure that the character is displayed immediately.
pub fn ft_putchar<W: Write>(mut w: W, c: char) {
	write!(w, "{}", c).unwrap();
	w.flush().unwrap();
}

#[macro_export]
macro_rules! ft_putchar {
	($c:expr) => {
		crate::ft_putchar::ft_putchar(std::io::stdout(), $c)
	};
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_ft_putchar_buffer() {
		let mut buf = Vec::new();
		ft_putchar(&mut buf, 'K');
		assert_eq!(buf, b"K");
}

	#[test]
	fn test_ft_putchar_unicode() {
		let mut buf = Vec::new();
		ft_putchar(&mut buf, '你');
		assert_eq!(std::str::from_utf8(&buf).unwrap(), "你");
	}
}
