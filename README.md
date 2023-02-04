# piscine-rust
This is originally a dump of the /app folder of the [test-rust docker image from 01-edu](https://github.com/01-edu/rust-tests/pkgs/container/test-rust).

It's the pedagogical and technical backbone of this 01-style piscine, which, unlike 42, is moulinette-only.  
Its general idea is simple :
- Start the first quest from the curriculum below
- Do the first exercise
- Test it with the included automated test
- If good, move on to the next one
- ...
- Congratulations, you are now a rust developer.

Though in the real program:
- each Day... uuh I mean, Quest becomes available the day after the other, 
- there is a 3 minutes cooldown between each moulinette test
- you're supposed to do the 2 rushes... uhh I mean, raids in groups of 3 and be audited manually by staff
- the practical project `chaikin` is (the only project) to be reviewed by peers.

# Instructions

Make sure the Rust toolchain is installed on your computer.  
It is best to install it the official way, by going [here](https://www.rust-lang.org/learn/get-started), and not through your package manager.

Make sure your whole piscine repo is the `solutions` folder alongside `tests`, at the root of this repository.  
This should look like this:  
`piscine-rust/ <- you are here!`  
`------------/tests`  
`------------/solutions/fibonacci2/<contents of the cargo module>`

You can launch the automated testing of a completed exercise at the root of this repository with the command `./test.sh <exercise_name>`.  
You can also directly run `cargo test --manifest-path tests/<exercise_name>_test/Cargo.toml`.

Feel free to fork this repository, remove `solutions/*` from `.gitignore` and get to work directly in it.  
Have fun!

# Notes

The original test mechanism was the following:
- The student repo was cloned under `/jail/student` by the 01 runner program
- The container is run, `$EXERCISE` and such are filled and `/jail` is set as current directory
- `/app/entrypoint.sh` (reworked into `test.sh`) is called upon running the container
- `/app/tests` is copied in `/jail` so that it is alongside the student repo
- `/jail/student` is renamed into `/jail/solutions` because runner clones the repo as "`student`" yet all rust tests are written to look for a "`solutions`" package :facepalm:  
	NB: I have yet no idea if the `/app/tests_utility` is necessary since it is never copied in `/jail/student` during testing


# Curriculum

## Week One

### Quest 01-rust

Video: https://www.youtube.com/watch?v=gjGzMMUdKDM

Exercises:

- fibonacci2 | https://github.com/01-edu/public/tree/master/subjects/fibonacci2
- scalar | https://github.com/01-edu/public/tree/master/subjects/scalar
- temperature_conv | https://github.com/01-edu/public/tree/master/subjects/temperature_conv
- looping | https://github.com/01-edu/public/tree/master/subjects/looping
- speed_transformation | https://github.com/01-edu/public/tree/master/subjects/speed_transformation
- groceries | https://github.com/01-edu/public/tree/master/subjects/groceries
- reverse_string | https://github.com/01-edu/public/tree/master/subjects/reverse_string
- find_factorial | https://github.com/01-edu/public/tree/master/subjects/find_factorial
- matrix_transposition | https://github.com/01-edu/public/tree/master/subjects/matrix_transposition
- division_and_remainder | https://github.com/01-edu/public/tree/master/subjects/division_and_remainder
- tuples_refs | https://github.com/01-edu/public/tree/master/subjects/tuples_refs

### Quest 02-rust

Video: https://www.youtube.com/watch?v=O0o19HANB_w

Exercices:

- ownership | https://github.com/01-edu/public/tree/master/subjects/ownership
- copy | https://github.com/01-edu/public/tree/master/subjects/copy
- borrow | https://github.com/01-edu/public/tree/master/subjects/borrow
- doubtful | https://github.com/01-edu/public/tree/master/subjects/doubtful
- borrow_me_the_reference | https://github.com/01-edu/public/tree/master/subjects/borrow_me_the_reference
- changes | https://github.com/01-edu/public/tree/master/subjects/changes
- string_literals | https://github.com/01-edu/public/tree/master/subjects/string_literals
- name_initials | https://github.com/01-edu/public/tree/master/subjects/name_initials
- arrange_it | https://github.com/01-edu/public/tree/master/subjects/arrange_it
- tic_tac_toe | https://github.com/01-edu/public/tree/master/subjects/tic_tac_toe

### Quest 03-rust

Video: https://www.youtube.com/watch?v=URAJIouRd0I

Exercices:

- circle | https://github.com/01-edu/public/tree/master/subjects/circle
- card_deck | https://github.com/01-edu/public/tree/master/subjects/card_deck
- arrays | https://github.com/01-edu/public/tree/master/subjects/arrays
- strings | https://github.com/01-edu/public/tree/master/subjects/strings
- edit_distance | https://github.com/01-edu/public/tree/master/subjects/edit_distance
- to_url | https://github.com/01-edu/public/tree/master/subjects/to_url
- capitalizing | https://github.com/01-edu/public/tree/master/subjects/capitalizing
- hashing | https://github.com/01-edu/public/tree/master/subjects/hashing
- string_permutation | https://github.com/01-edu/public/tree/master/subjects/string_permutation
- bigger | https://github.com/01-edu/public/tree/master/subjects/bigger
- simple_hash | https://github.com/01-edu/public/tree/master/subjects/simple_hash
- collect | https://github.com/01-edu/public/tree/master/subjects/collect

### Raid 01-rust

Exercice:

- drawing | https://github.com/01-edu/public/tree/master/subjects/drawing

---

## Week Two

### Quest 04-rust

Video: https://www.youtube.com/watch?v=3D0mOw0egHc

Exercices:

- unwrap_or_expect | https://github.com/01-edu/public/tree/master/subjects/unwrap_or_expect
- panic | https://github.com/01-edu/public/tree/master/subjects/panic
- handling | https://github.com/01-edu/public/tree/master/subjects/handling
- profanity_filter | https://github.com/01-edu/public/tree/master/subjects/profanity_filter
- question_mark | https://github.com/01-edu/public/tree/master/subjects/question_mark
- banner | https://github.com/01-edu/public/tree/master/subjects/banner
- cipher | https://github.com/01-edu/public/tree/master/subjects/cipher
- error_types | https://github.com/01-edu/public/tree/master/subjects/error_types
- boxing_todo | https://github.com/01-edu/public/tree/master/subjects/boxing_todo

### Quest 05-rust

Video: https://www.youtube.com/watch?v=XyUliQD7v-0

Exercices:

- middle_day | https://github.com/01-edu/public/tree/master/subjects/middle_day
- does_it_fit | https://github.com/01-edu/public/tree/master/subjects/does_it_fit
- macro_calculator | https://github.com/01-edu/public/tree/master/subjects/macro_calculator
- shopping_mall | https://github.com/01-edu/public/tree/master/subjects/shopping_mall
- expected_variable | https://github.com/01-edu/public/tree/master/subjects/expected_variable
- mobs | https://github.com/01-edu/public/tree/master/subjects/mobs

### Raid 02-rust

Exercice:

- road-intersection | https://github.com/01-edu/public/tree/master/subjects/road-intersection

---

## Week Three

### Quest 06-rust

Video: https://www.youtube.com/watch?v=xCEDVb4p7Js

Exercices:

- generics | https://github.com/01-edu/public/tree/master/subjects/generics
- traits | https://github.com/01-edu/public/tree/master/subjects/traits
- lifetimes | https://github.com/01-edu/public/tree/master/subjects/lifetimes
- lalgebra_scalar | https://github.com/01-edu/public/tree/master/subjects/lalgebra_scalar
- matrix | https://github.com/01-edu/public/tree/master/subjects/matrix
- matrix_ops | https://github.com/01-edu/public/tree/master/subjects/matrix_ops
- matrix_mult | https://github.com/01-edu/public/tree/master/subjects/matrix_mult
- lalgebra_vector | https://github.com/01-edu/public/tree/master/subjects/lalgebra_vector
- blood_types | https://github.com/01-edu/public/tree/master/subjects/blood_types
- border_cross | https://github.com/01-edu/public/tree/master/subjects/border_cross
- roman_numbers | https://github.com/01-edu/public/tree/master/subjects/roman_numbers
- roman_numbers_iter | https://github.com/01-edu/public/tree/master/subjects/roman_numbers_iter
- vectors_operations | https://github.com/01-edu/public/tree/master/subjects/vector_operations
- events | https://github.com/01-edu/public/tree/master/subjects/events
- delete_prefix | https://github.com/01-edu/public/tree/master/subjects/delete_prefix
- commits_stats | https://github.com/01-edu/public/tree/master/subjects/commits_stats


### Quest 07-rust

Video: https://www.youtube.com/watch?v=jzgKZTtVNwQ

Exercices:

- box_it | https://github.com/01-edu/public/tree/master/subjects/box_it
- borrow_box | https://github.com/01-edu/public/tree/master/subjects/borrow_box
- box_recursion | https://github.com/01-edu/public/tree/master/subjects/box_recursion
- how_many_references | https://github.com/01-edu/public/tree/master/subjects/how_many_references
- ref_cell | https://github.com/01-edu/public/tree/master/subjects/ref_cell
- drop_the_thread | https://github.com/01-edu/public/tree/master/subjects/drop_the_thread

### Project 01-rust

Exercice:
- chaikin | https://github.com/01-edu/public/tree/master/subjects/chaikin

---

## Week Four

### Quest 08-rust

Video: https://www.youtube.com/watch?v=sdEEmmlI6K0

Exercices:

- closures | https://github.com/01-edu/public/tree/master/subjects/closures
- sales | https://github.com/01-edu/public/tree/master/subjects/sales
- adding | https://github.com/01-edu/public/tree/master/subjects/adding
- adding_twice | https://github.com/01-edu/public/tree/master/subjects/adding_twice
- get_products | https://github.com/01-edu/public/tree/master/subjects/get_products
- highest | https://github.com/01-edu/public/tree/master/subjects/highest
- iterators | https://github.com/01-edu/public/tree/master/subjects/iterators
- slices_to_map | https://github.com/01-edu/public/tree/master/subjects/slices_to_map
- step_iterator | https://github.com/01-edu/public/tree/master/subjects/step_iterator
- project_motion | https://github.com/01-edu/public/tree/master/subjects/project_motion

### Quest 09-rust

Video: https://www.youtube.com/watch?v=LMOcoPamcxM

Exercices:

- stars | https://github.com/01-edu/public/tree/master/subjects/stars
- ordinal | https://github.com/01-edu/public/tree/master/subjects/ordinal
- pangram | https://github.com/01-edu/public/tree/master/subjects/pangram
- diamond_creation | https://github.com/01-edu/public/tree/master/subjects/diamond_creation
- scores | https://github.com/01-edu/public/tree/master/subjects/scores
- talking | https://github.com/01-edu/public/tree/master/subjects/talking
- searching | https://github.com/01-edu/public/tree/master/subjects/searching
- logic_number | https://github.com/01-edu/public/tree/master/subjects/logic_number
- rot | https://github.com/01-edu/public/tree/master/subjects/rot
- pig_latin | https://github.com/01-edu/public/tree/master/subjects/pig_latin
- spelling | https://github.com/01-edu/public/tree/master/subjects/spelling
- rgb_match | https://github.com/01-edu/public/tree/master/subjects/rgb_match
