// JavaScript Object Macro Examples

! version = 2.0

> object setvar javascript
	// Example of how to get the current user's ID and set
	// variables for them.
	uid   = rs.currentUser();
	name  = args.shift();
	value = args.join(" ");
	rs.setUservar(uid, name, value);
< object

> object add javascript
	// Demonstrats that JS objects can return numbers.
	var a = args[0];
	var b = args[1];
	return parseInt(a) + parseInt(b);
< object

+ add # and #
- <star1> + <star2> = <call>add <star1> <star2></call>

+ javascript set * to *
- Set user variable <star1> to <star2>.<call>setvar <star1> <star2></call>
