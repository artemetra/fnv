**Warning: this text is still just a draft and mostly used by myself**

# **Research into reverse engineering FL Studio's .fnv file format**

## What is fnv?

.fnv (most likely standing for '**f**ruity e**nv**elope') - is a proprietary file format for saving curves, envelopes, graphs and other connected point content (also called states) inside FL Studio. Here are some examples of curves you might have encountered in FL:

![envelope](https://github.com/artemetra/fnv/blob/main/res/envelope_example.png?raw=true "envelope_example.png")

![lfo](https://github.com/artemetra/fnv/blob/main/res/lfo_example.png?raw=True "lfo_example.png")

![graph](https://github.com/artemetra/fnv/blob/main/res/graph_example.png?raw=true "graph_example.png")

![automation_clip](https://github.com/artemetra/fnv/blob/main/res/automation_example.png?raw=true "automation_example.png")

Apart from being just the examples, these are actually the only types of curves to exist in FL, they are:

* Envelope
* LFO Envelope
* Graph
* Automation clips.*

  ##### \* - automation clips can't actually be saved in fnv files directly, but their _states_ can be copied and pasted inside FL. More on   that later

All of them have their disctinctive features and are saved in .fnv's differently. I analyze them using a hex editor, so expect a lot of byte values and adresses. First of all, let's look at what is the same for all types:

## _The Similarities:_

1. ### Contents of curves

    Each curve is made of **points**, which are connected with each other by **connections(?)**. These connections can be modified and customised by changing their **modes** and their **tensions.** Connections cannot overlap vertically, however instant jumps, where 2 or more points have same x coordinates are allowed.  

2. ### Point modes

    There are a number of ways you can connect 2 points inside the curves. These ways will are called **modes**. Here's a list of them in FL:

    ![modes](https://github.com/artemetra/fnv/blob/main/res/modes_example.png?raw=true "modes_example.png")

    And in text:

    | Name           | Short* | Byte   |
    | --------       | -----  | :--:   |
    | Hold           | HD     | `02`   |
    | Smooth         | SM     | `0A`   |
    | Single Curve   | SC     | `00`   |
    | Single Curve 2 | SC2    | `07`   |
    | Single Curve 3 | SC3    | `0B`   |
    | Double Curve   | DC     | `01`   |
    | Double Curve 2 | DC2    | `01`   |
    | Double Curve 3 | DC3    | `08`   |
    | Half Sine      | HS     | `09`   |
    | Stairs         | ST     | `03`   |
    | Smooth Stairs  | SST    | `04`   |
    | Pulse          | PU     | `05`   |
    | Wave           | WV     | `06`   |

    *short names are unofficial  

    As you can see, the bytes aren't in the correct order. Here's a different version of this table, sorted by byte.

    | Byte   | Name           | Short  |
    | :--:   | --------       | -----  |
    | `00`   | Single Curve   | SC     |
    | `01`   | Double Curve   | DC     |
    | `02`   | Hold           | HD     |
    | `03`   | Stairs         | ST     |
    | `04`   | Smooth Stairs  | SST    |
    | `05`   | Pulse          | PU     |
    | `06`   | Wave           | WV     |
    | `07`   | Single Curve 2 | SC2    |
    | `08`   | Double Curve 2 | DC2    |
    | `09`   | Half Sine      | HS     |
    | `0A`   | Smooth         | SM     |
    | `0B`   | Single Curve 3 | SC3    |
    | `0С`   | Double Curve 3 | DC3    |

    (this might be the order in which the modes were implemented!)

    Modes specify how a connection should be built from the _left_ side of a point (connecting the current one and the previous one, not the next)  

3. ### Common FNV file structure facts

    _*(This section is highly likely to be changed and will be edited a lot)_  

    * 1st byte defines the curve type
    * Points are saved in chunks of **24 bytes,** which makes it possible to get the number of points in a FNV using only the fnv file size
	* The 0x8th-0xBth bytes represent the number of points in a curve (in little endian)
	* The x coordinates of points are _not_ absolute, but are relative to the previous point (the x coordinate in the FNV itself stores an offset, rather than a coordinate). This prevents the connections from overlapping
	* The maximum number of points is the unsinged 32 bit integer limit, being 4294967295
	* The last 32 bytes (the "footer") define global settings, such as being on/off, lfo phase 
	

## _The Differences:_

## Envelope

![envelope](https://github.com/artemetra/fnv/blob/main/res/envelope_on.png?raw=true "envelope_on.png")

First byte in header: `01`  

Minimum number of points: 1

Number of points formula:  
`N = (F-72)/24+1`,
where N - number of points, F - file size in bytes

In the footer, the first 4 bytes are FF FF FF FF (they might be a separator, however, as we may see now, they can also represent global values).
By default, the next 3 chunks of 4 bytes are all FF, however, they actually specify which points have the ADSR section flags. First chunks is for Decay, second is for Loop start, and the third is for Sustain / Loop end (the list matches the order of the options in FL). Each chuck is just an index of a point that has these values turned on, and if none do, it goes to the default value (4 FF bytes)

## LFO Envelope

Minimum number of points: 1

First byte in header: `02`  
Byte `-28`: tempo/global flags:  

| Byte | Tempo | Global |
| :--: | :---: | :----: |
| `00` | Off   | Off    |
| `01` | On    | Off    |
| `02` | Off   | On     |
| `03` | On    | On     |

Bytes `-4:-1`: LFO phase shift  

Number of points formula:  
`N = (F-76)/24`,
where N - number of points, F - file size in bytes

| Byte | Frozen | Bipolar |
| :--: | :---: | :----: |
| `07` | Off   | Off    |
| `0F` | On    | Off    |
| `03` | Off   | On     |
| `0B` | On    | On     |


## Graph  
First byte in header: `03`

Minimum number of points: 2

Number of points formula:  
`N = (F-80)/24+2`,
where N - number of points, F - file size in bytes

|Num |Byte  |Diff|
|:-: |:-:   |:-: |
| 1  | `B0` | -  |
| 2  | `C0` | 16 |
| 3  | `C8` |  8 |
| 4  | `D0` |  8 |
| 5  | `D4` |  4 |
| 6  | `D8` |  4 |
| 7  | `DC` |  4 |
| 8  | `E0` |  4 |
| 10 | `E2` |  2 |
| 11 | `E4` |  2 |
| 12 | `E8` |  2 |
| 13 | `EA` |  2 |
| 14 | `EC` |  2 |
| 15 | `EE` |  2 |
| 16 | `F0` |  1 |
