# When Expressions Are Not Regular

Often if you are evaluating a string like an email, phone number or even a password then a regular expression is a great choice. I have used RegEx in this capacity many times but sometimes I reach for this tool when something else would be a much better fit. The best example I can think of is evaluating code with RegEx. 
If your problem is simple enough you can get away with doing this but once you need something like a nested pattern it will become complicated very quickly. Regular Expressions are usually implemented as Finite Automata which cannot support a potentially infinite state like the nesting we see in our programming languages.
If your RegEx implementation uses backtracking then it is possible to write a nested pattern but backtracking is slow and not all RegEx implementations support it. So, if you need to evaluate a string of code with nested brackets or parenthesis what can you do? You can implement a toy parser.

# Parsers Are Easier Than You Think

Writing a parser just to evaluate some code for a simple string replacement probably sounds like a lot of work. It might be, depending on your needs, but if you're hacking something together just for your own use case it's not that hard. In my case I needed a tool to automate transitioning from the Classic Asserts of NUnit 3 to NUnit 4's new Assert.That syntax.
At first I spent a lot of time trying to solve this problem with RegEx and initially it worked fine. Right up until I ran into Asserts that took the results of methods as parameters. Now I needed to handle capturing up until a close parenthesis unless another open parenthesis had appeared. This is where I encountered the issue of finite state automata not handling infinitely nested patterns.
My new approach involves searching for the beginning of my Assert string eg. "Assert.AreEqual(" and then counting the number of parenthesis that are currently open. Once a comma is encountered I'll know based on how many open parenthesis we have whether or not this is still part of the same parameter or a new one.

In this way we can grab all of the parameters to the function and then handle outputting our new string. So Assert.AreEqual(actual, expected); becomes Assert.That(expected, Is.EqualTo(actual)); A side note: NUnit also flipped the order of parameters for actual vs expected in the new syntax. This new approach can easily handle my use case of replacing the old asserts with the new ones while also being performant and easy to read.
If you would like to check out the code or use the tool for yourself you can find it [here](https://github.com/Sorrien/nunit_update_utility)

```rust
         for next_char in &chars[i..] {
            match *next_char {
                '(' => {
                    if parenthesis_depth > 0 {
                        parameter.push(*next_char);
                    }
                    parenthesis_depth += 1;
                }
                ')' => {
                    if parenthesis_depth == 1 {
                        parameters.push(parameter.clone());
                        parameter.clear();
                    } else {
                        parameter.push(*next_char);
                        parenthesis_depth -= 1;
                    }
                }
                ',' => {
                    if parenthesis_depth == 1 {
                        parameters.push(parameter.clone());
                        parameter.clear();
                        if chars[i + 1].is_ascii_whitespace() {
                            i += 1;
                        }
                    } else {
                        parameter.push(*next_char);
                    }
                }
                ';' => {
                    break;
                }
                _ => {
                    if in_angle_brackets {
                        generic.push(*next_char);
                    } else {
                        parameter.push(*next_char);
                    }
                }
            }

            i += 1;
        }
```