# **Research into reverse engineering FL Studio's .fnv file format**
## What is fnv?

.fnv (most likely standing for '**f**ruity e**nv**elope') - is a proprietary file format for saving curves, envelopes and 'states' inside FL Studio. Here are some examples of curves you might have encountered in FL: 


![envelope](url)

![lfo](url)

![graph](url)

![automation_clip](url)


Apart from being just the examples, these are actually the only known types of curves to exist in FL, they are:
* Envelope
* LFO Envelope
* Graph
* Automation clips.*

  ##### * - automation clips can't actually be saved in fnv files directly, but their states can be copied and pasted inside FL. More on that later.

All of them have their disctinctive features and are saved in .fnv's differently. Let's look at the differences between each one:

## Envelopes

![envelope](url)


