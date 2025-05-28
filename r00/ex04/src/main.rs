/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: welee <welee@student.42singapore.sg>       +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2025/05/28 17:16:06 by welee             #+#    #+#             */
/*   Updated: 2025/05/28 18:18:15 by welee            ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use ex04::ft_putchar;
use ex04::ft_is_negative;

fn main() {
	ft_is_negative!(-42);
	ft_putchar!('\n');
	ft_is_negative!(42);
	ft_putchar!('\n');
	ft_is_negative!(0);
	ft_putchar!('\n');
}
