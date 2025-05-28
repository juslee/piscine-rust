/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_is_negative.rs                                  :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: welee <welee@student.42singapore.sg>       +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/28 17:12:13 by welee             #+#    #+#             */
/*   Updated: 2025/05/28 18:00:54 by welee            ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io::Write;
use crate::ft_putchar::ft_putchar;

/// Prints 'N' if the number is negative, otherwise prints 'P'.
pub fn ft_is_negative<W: Write>(mut w: W, n: i32) {
	if n < 0 {
		ft_putchar(&mut w, 'N');
	} else {
		ft_putchar(&mut w, 'P');
	}
}

#[macro_export]
macro_rules! ft_is_negative {
	($n:expr) => {
		$crate::ft_is_negative::ft_is_negative(std::io::stdout(), $n)
	};
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_ft_is_negative() {
		let mut buf: Vec<u8> = Vec::new();
		ft_is_negative(&mut buf, -5);
		assert_eq!(buf, b"N");

		buf.clear();
		ft_is_negative(&mut buf, 5);
		assert_eq!(buf, b"P");
	}

	#[test]
	fn test_ft_is_negative_zero() {
		let mut buf: Vec<u8> = Vec::new();
		ft_is_negative(&mut buf, 0);
		assert_eq!(buf, b"P");
	}
}
