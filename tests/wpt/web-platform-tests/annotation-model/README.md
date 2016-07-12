Annotation-model: Tests for the Web Annotation Data Model
=========================================================

The [Web Annotation Data Model](https://www.w3.org/TR/annotation-model)
specification presents a JSON-oriented collection of terms and structure that
permit the sharing of annotations about other content.

The purpose of these tests is to help validate that each of the structural
requirements expressed in the Data Model specification are properly supported
by implementations.

The general approach for this testing is to enable both manual and automated
testing. However, since the specification has no actual user interface
requirements, there is no general automation mechanism that can be presented
for clients.  Instead, the automation mechanism is one where client
implementors could take advantage of the plumbing we provide here to push their
data into the tests and collect the results of the testing.  This assumes
knowledge of the requirements of each test / collection of tests so that the
input data is relevant.  Each test or test collection contains information
sufficient for the task.

Running Tests
-------------

In the case of this test collection, we will be initially creating manual
tests.  These will automatically determine pass or fail and generate output for
the main WPT window.  The plan is to minimize the number of such tests to
ease the burden on the testers while still exercising all the features.

The workflow for running these tests is something like:

1. Start up the test driver window and select the annotation-model tests -
   click "Start"
2. A window pops up that shows a test - the description of which tells the
   tester what input is expected.  The window contains a textarea into which
   the input can be typed / pasted, along with a button to click to start
   testing that input.
3. The tester (presumably in another window) brings up their annotation client
   and uses it to generate an annotation that supplies the requested structure.
   They then copy / paste that into the aforementioned textarea and select the
   button.
4. The test runs.  Success or failure is determined and reported to the test
   driver window, which then cycles to the next test in the sequence.
5. Repeat steps 2-4 until done.
6. Download the JSON format report of test results, which can then be visually
   inspected, reported on using various tools, or passed on to W3C for
   evaluation and collection in the Implementation Report via github.

**Remember that while these tests are written to help exercise implementations,
their other (important) purpose is to increase confidence that there are
interoperable implementations.** So, implementers are our audience, but these
tests are not meant to be a comprehensive collection of tests for a client that
might implement the Recommendation.  The bulk of the tests are manual because
there are no UI requirements in the Recommendation that would make it possible
to effectively stimulate every client portably.

Having said that, because the structure of these "manual" tests is very rigid,
it is possible for an implementer who understands test automation to use an
open source tool such as [Selenium](http://www.seleniumhq.org/) to run these
"manual" tests against their implementation - exercising their implementation
against content they provide to create annotations and feed the data into our
test input field and run the test.

Capturing and Reporting Results
-------------------------------

As tests are run against implementations, if the results of testing are
submitted to [test-results](https://github.com/w3c/test-results/) then they will
be automatically included in documents generated by
[wptreport](https://www.github.com/w3c/wptreport). The same tool can be used
locally to view reports about recorded results.


Automating Test Execution
-------------------------

Writing Tests
-------------

If you are interested in writing tests for this environment, see the
associated [CONTRIBUTING](CONTRIBUTING.md) document.