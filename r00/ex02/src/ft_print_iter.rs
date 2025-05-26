/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_print_iter.rs                                   :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: welee <welee@student.42singapore.sg>       +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/26 15:37:00 by welee             #+#    #+#             */
/*   Updated: 2025/05/26 15:53:18 by welee            ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io::Write;
use crate::ft_putchar;

pub fn ft_print_iter<W, I>(mut w: W, iter: I)
where
	W: Write,
	I: IntoIterator,
	I::Item: Into<char>,
{
	for c in iter {
		ft_putchar::ft_putchar(&mut w, c.into());
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_ft_print_iter() {
		let mut buf = Vec::new();
		ft_print_iter(&mut buf, (b'a'..=b'z').rev());
		assert_eq!(buf, b"zyxwvutsrqponmlkjihgfedcba");
	}

	#[test]
	fn test_ft_print_iter_unicode() {
		let mut buf = Vec::new();
		ft_print_iter(&mut buf, 'a'..='z');
		assert_eq!(std::str::from_utf8(&buf).unwrap(), "abcdefghijklmnopqrstuvwxyz");
	}
}


