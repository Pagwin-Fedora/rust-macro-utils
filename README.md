# rust-macro-utils

This repository was made to respond to a joking comment that python decorators are better than rust macros by implementing most of python's decorator functionality into a rust macro. Warning this is a buggy mess that will have issues with async functions and will provide annoying(but working) output for functions that use any visibility other than fully public or fully private aka `pub` or `pub(self)`.
