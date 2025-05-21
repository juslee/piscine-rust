/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_print_alphabet.rs                               :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: welee <welee@student.42singapore.sg>       +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/03/15 10:43:55 by welee             #+#    #+#             */
/*   Updated: 2024/03/17 13:36:51 by welee            ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io::Write;

pub fn ft_putchar(c: char) {
	write!(std::io::stdout(), "{}", c).unwrap();
}

pub fn ft_print_alphabet() {
	let mut c = 'a';
	while c <= 'z' {
		ft_putchar(c);
		c = (c as u8 + 1) as char;
	}
}
