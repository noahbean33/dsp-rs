# **Foundational Requirements and Pedagogical Frameworks for Embedded Systems Digital Image Processing Textbooks**

The contemporary landscape of digital image processing has undergone a significant transformation, migrating from the sanctuary of high-performance workstations to the highly constrained environments of embedded systems.1 This shift necessitates a fundamental re-evaluation of the educational materials used to train the next generation of engineers. A textbook serving this discipline must not only cover the theoretical underpinnings of computer vision but must also provide an exhaustive treatment of the hardware-software co-design principles that allow these algorithms to function within the bounds of limited power, memory, and computational cycles.1 The complexity of modern visual tasks—ranging from real-time lane detection in autonomous vehicles to gesture recognition on ultra-low-power microcontrollers—requires a curriculum that bridges the gap between abstract mathematical models and silicon-level implementation.3

## **Architectural Paradigms and Hardware Considerations**

The most critical distinction of an embedded vision textbook is its emphasis on hardware architecture. Unlike general-purpose software development, embedded image processing is inextricably linked to the physical platform on which it resides.1 A modern textbook must offer a comprehensive analysis of the various processing architectures available, including Digital Signal Processors (DSPs), Field-Programmable Gate Arrays (FPGAs), Application-Specific Integrated Circuits (ASICs), and Graphics Processing Units (GPUs).2

FPGAs, in particular, are central to high-performance embedded vision due to their ability to exploit fine-grained parallelism.5 A textbook must detail the internal components of these devices, such as Configurable Logic Blocks (CLBs), specialized DSP slices, and block RAM (BRAM).2 The narrative should guide the reader through the differences between temporal parallelism, where tasks are organized in a pipeline, and spatial parallelism, where data is distributed across multiple identical processing units.5 This distinction is vital for optimizing throughput and reducing the latency that characterizes real-time vision systems.2

| Hardware Platform | Architectural Strategy | Primary Constraint | Target Application Domain |
| :---- | :---- | :---- | :---- |
| **ASIC** | Fixed Logic | Development Cost | High-volume consumer electronics 5 |
| **FPGA** | Reconfigurable Logic | Power Density | Prototyping and low-latency robotics 2 |
| **DSP** | Instruction-Level | Memory Bandwidth | Real-time signal filtering and ADAS 1 |
| **GPU** | Data-Parallel | Power Consumption | Deep learning and heavy compute tasks 1 |
| **NPU** | Model-Specific | Flexibility | Edge-AI and TinyML applications 12 |

Beyond the processing core, the textbook must address the "memory wall," which refers to the discrepancy between the speed of the processor and the bandwidth of external memory.5 In embedded vision, where data rates for high-definition video can be immense, managing memory access is a primary design constraint.2 Techniques such as row buffering—storing only the necessary lines of an image to perform local operations—are essential topics for any student of this field.2

## **Computational Foundations and Fixed-Point Arithmetic**

One of the most significant pedagogical challenges in transitioning students to embedded systems is the move from floating-point arithmetic to fixed-point representations.1 Most embedded processors and hardware accelerators lack the silicon area to support efficient floating-point units (FPUs).1 Consequently, a textbook must dedicate substantial space to fixed-point mathematics, precision loss analysis, and overflow management.1

The implementation of elementary functions that are standard in high-level libraries—such as square roots, trigonometric functions, and exponentials—requires specialized algorithms when implemented in hardware.5 The COordinate Rotation Digital Computer (CORDIC) algorithm is a cornerstone of this domain, allowing for the calculation of these functions using only shifts and additions.2 A curriculum must explain the iterative nature of CORDIC and its hardware efficiency relative to polynomial approximations or lookup table (LUT) implementations.2

| Computational Technique | Resource Requirement | Mathematical Accuracy | Hardware Utility |
| :---- | :---- | :---- | :---- |
| **Fixed-Point** | Integer Logic | Variable (Scaling) | Universal in embedded hardware 1 |
| **CORDIC** | Shift/Add Logic | High (Iterative) | Trig/Hyperbolic functions in FPGAs 2 |
| **Lookup Table** | Memory (BRAM/ROM) | High (Sampled) | Non-linear point operations 2 |
| **Poly Approx.** | Multipliers | High (High Order) | Smooth curve fitting 2 |

Advanced number systems, such as the Logarithmic Number System (LNS) or the emerging Posit number format, should be introduced as alternatives to traditional two's complement arithmetic.2 These systems can offer wider dynamic ranges and better precision for certain vision tasks, such as those found in medical imaging or high-dynamic-range (HDR) processing.2

## **Image Acquisition and Sensor Interfacing**

An embedded system's interaction with the physical world begins at the sensor interface.1 A comprehensive textbook must move beyond theoretical images to discuss the realities of image sensors, including CMOS and CCD architectures, their readout styles, and the noise characteristics inherent in digital sensing.1

The interface between the camera and the processing unit is a frequent bottleneck.1 The curriculum should cover industry-standard protocols such as the MIPI Camera Serial Interface (CSI), Camera Link, and GigE Vision.2 These protocols involve high-speed signaling, often utilizing Low-Voltage Differential Signaling (LVDS) to ensure data integrity over physical connections.1 Understanding the "Camera Processing Pipeline"—which includes de-mosaicing (Bayer filtering), white balance adjustment, and gamma correction—is critical for students who will be responsible for the entire vision system from photon to decision.2

### **Communication and Output Protocols**

Embedded vision systems rarely operate in isolation.3 They must communicate with other controllers, display results to users, or transmit data to a central cloud server.2 A textbook must therefore cover:

* **Low-Speed Control**: I2C and SPI for sensor configuration and peripheral control.2  
* **High-Speed Data**: USB 3.0, Ethernet, and PCI Express for high-bandwidth video transfer.2  
* **Visual Output**: Display drivers and content generation for HDMI, VGA, or integrated LCD panels.2  
* **System Integration**: Bus architectures such as Altera’s Avalon or ARM’s AXI that facilitate the interconnection of custom accelerators and CPU cores.2

## **Classical Image Processing in Hardware**

While modern vision often emphasizes high-level analytical tasks, the foundation remains built upon low-level image processing operations.14 These operations are typically categorized based on their spatial data access patterns.2

### **Point Operations**

Point operations are the simplest to implement in hardware, as the output pixel value is a function only of the input pixel at the same coordinate.2 A textbook should discuss the implementation of contrast enhancement, brightness adjustment, and global thresholding using simple arithmetic or lookup tables.2 Color space conversions—moving between RGB, YUV, and HSV—are linear transformations that are ubiquitous in video processing and should be treated with mathematical rigor.2

### **Local and Neighborhood Operations**

The shift to neighborhood operations—where the output depends on a small window of input pixels—introduces significant complexity.2 Linear filtering (convolution) and non-linear filtering (median filters) are the primary tools for noise reduction and edge detection.2 The curriculum must teach the implementation of sliding windows and line buffers, which allow the system to maintain a local window of data while the image streams through the processor.2

A critical optimization for hardware designers is "separability".5 If a 2D filter can be decomposed into two 1D filters, the number of required operations drops from ![][image1] to ![][image2], which can save thousands of logic gates and significantly reduce power consumption.5 Morphological operations—erosion, dilation, opening, and closing—are another vital topic, particularly for cleaning up binary images in industrial inspection or document analysis tasks.1

### **Global Operations and Transforms**

Global operations, such as the Histogram or the Fast Fourier Transform (FFT), require access to the entire image and present unique challenges for streaming hardware.2 A textbook should cover the trade-offs between frame-buffered processing and streaming implementations that may require multiple passes over the data.1

## **Deep Learning at the Edge: TinyML and Model Optimization**

The most significant recent addition to the embedded vision curriculum is the migration of deep learning models to resource-constrained devices.4 This "TinyML" revolution requires students to understand not just how to train a model, but how to shrink it to fit on a microcontroller or a small FPGA.4

The optimization pipeline is a central theme in modern embedded vision literature.12 Students must master the following techniques:

* **Network Pruning**: Identifying and removing weights or entire neurons that do not significantly contribute to the model's accuracy, thereby reducing both memory footprint and compute time.12  
* **Weight and Activation Quantization**: Moving from 32-bit floating-point values to 8-bit or even 4-bit integers.4 The narrative should explain the impact of quantization on the "loss landscape" and how quantization-aware training can mitigate accuracy degradation.12  
* **Knowledge Distillation**: Training a compact "student" model to mimic the outputs of a complex "teacher" model, allowing for high-performance inference on low-power hardware.12  
* **Neural Architecture Search (NAS)**: Automating the design of network topologies that are specifically optimized for a target hardware's latency and power constraints.12

| Optimization Phase | Method | Hardware Impact | Insight |
| :---- | :---- | :---- | :---- |
| **Model Design** | NAS / Manual Tuning | Reduced operations | Tailors model to specific silicon 12 |
| **Compression** | Pruning / Distillation | Lower memory | Reduces weight transfer overhead 12 |
| **Conversion** | Quantization | Integer acceleration | Enables TinyML on microcontrollers 4 |
| **Deployment** | TFLite / HLS | Lower power | Optimizes for specific AI accelerators 4 |

The software ecosystem surrounding TinyML is equally important.4 A textbook must introduce frameworks like TensorFlow Lite, OpenCL, and High-Level Synthesis (HLS) tools that bridge the gap between high-level Python models and low-level hardware implementations.15

## **Design Methodology and Optimization Strategies**

A professional-level textbook must go beyond listing algorithms and instead focus on a rigorous design methodology.2 The design of an embedded vision system is a constant balancing act between performance, power, area, and cost.2

### **Hardware-Software Partitioning**

One of the most difficult decisions an engineer faces is deciding which parts of a vision pipeline to implement in dedicated hardware (for speed and efficiency) and which to leave in software (for flexibility and ease of development).2 A curriculum should provide frameworks for making these decisions, utilizing profiling tools to identify the most computationally intensive kernels that would benefit from hardware acceleration.1

### **Timing, Synchronization, and Clock Domains**

Real-time systems are defined by their timing.2 A textbook must cover low-level hardware concepts such as pipelining to increase clock frequency and the synchronization techniques required when crossing between different clock domains—such as an image sensor running at 74.25 MHz and a processor core running at 200 MHz.2

### **High-Level Synthesis (HLS)**

The adoption of High-Level Synthesis (HLS) has transformed the way FPGAs are programmed for image processing.5 By allowing developers to write in C or C++ and synthesize that code into hardware, HLS significantly speeds up the development cycle.5 However, the textbook must emphasize that HLS is not "magic"—a hardware-aware approach is required to achieve efficient results.5 Topics should include loop unrolling, pipelining directives, and memory partitioning to maximize throughput.18

## **Advanced Vision Tasks and Analytical Pipelines**

Moving up the abstraction ladder, an embedded systems textbook must address how low-level pixels are transformed into high-level semantics.14 This transition involves feature extraction, object detection, and event analysis.1

### **Feature Extraction and Tracking**

Literature highlights several algorithms that have become standards for feature extraction in embedded systems:

* **Scale-Invariant Feature Transform (SIFT)**: While computationally expensive, its robustness makes it a frequent target for acceleration.1  
* **Maximally Stable Extremal Regions (MSER)**: Used for finding stable blobs in images, often for text or object recognition.1  
* **Histograms of Oriented Gradients (HoG)**: A cornerstone of human detection and handwriting recognition, particularly in embedded contexts using OpenCV and Python.1

Once features are identified, they must often be tracked over time.1 Motion History Histograms and multimodal background modeling are techniques used to recognize human actions and detect events in surveillance video.1

### **3D Vision and Stereo Matching**

Implementing 3D vision on embedded devices is a significant technical challenge.3 Stereo matching algorithms, such as those based on the Sum of Absolute Differences (SAD), allow a system to estimate depth by comparing images from two cameras.1 A textbook should discuss the FPGA architectures used to perform these calculations in real-time, enabling applications like obstacle avoidance in robotics or hand-tracking in augmented reality (AR) systems.1

## **Educational Standards and Global Curricula**

An analysis of syllabi from leading technical universities and international education boards provides a clear picture of what a comprehensive curriculum should entail.7

### **The Academic Core**

Most master's level programs in embedded systems, such as those at Anna University or Sri Sairam Engineering College, structure their vision courses around a progression from digital principles to complex system design.6

| Module | Core Topics | Recommended Learning Outcomes |
| :---- | :---- | :---- |
| **Foundation** | Human visual system, DIP fundamentals | Understanding biological vs. digital vision 7 |
| **Hardware** | FPGA vs ASIC, RISC architectures, RTOS | Mastery of the underlying processing units 6 |
| **Algorithms** | Segmentation, compression, HoG | Proficiency in classical vision kernels 6 |
| **Tools** | OpenCV, TensorFlow Lite, VHDL/Verilog | Practical skill in software and hardware languages 6 |
| **Systems** | ADAS, diagnostic apps, IoT security | Ability to design full-cycle vision solutions 6 |

University programs at MIT, Stanford, Berkeley, and CMU increasingly emphasize "project-driven" learning.17 A curriculum should guide students through the creation of tangible systems, such as dog fitness trackers, automated bird feeders, or autonomous robotic platforms.15 These projects require students to integrate data fetching, engineering, model selection, and hyperparameter tuning into a cohesive workflow.16

### **Emerging Curricular Trends**

Since 2024, over 70% of AI bachelor's programs have updated their curricula to include specialized modules on real-time analytics for robotics, autonomous vehicles, and healthcare diagnostics.17 This indicates that a textbook must stay current with industry demands, particularly in the areas of SLAM (Simultaneous Localization and Mapping), edge-based federated learning, and reinforcement learning for robotic control.15

## **Industrial and Consumer Application Case Studies**

To contextualize the technical material, a textbook should include detailed case studies of successful embedded vision products.3

### **Automotive Safety and ADAS**

The automotive industry is perhaps the most demanding consumer of embedded vision technology.1 Systems like Lane Departure Warning (LDW) and Lane Keep Assistant (LKA) must operate with near-perfect reliability on fixed-point DSPs.1 The narrative should explore the numerical requirements and environmental challenges (e.g., shadows, rain, glaring headlights) that these systems must overcome.1

### **Consumer Robotics and the Optical Mouse**

The optical mouse is one of the most successful high-volume embedded vision systems ever created.3 It demonstrates how a very low-resolution sensor combined with a specialized processor for cross-correlation can perform precise motion estimation at a extremely low cost.3 Similarly, the evolution of consumer vacuum robots from simple bump-sensors to visual-SLAM highlights the increasing complexity of embedded vision in the home.3

### **Medical and Cultural Heritage Applications**

Embedded vision also plays a critical role in specialized fields such as dermoscopy for skin cancer detection and the preservation of cultural heritage through the restoration and reconstruction of ancient artworks.14 These applications often require a deeper understanding of image restoration, denoising, and graph-based image analysis.14

## **Ethical Considerations and Future Directions**

As embedded vision becomes integrated into the fabric of daily life—from smart doorbells to facial recognition in public spaces—the curriculum must address the ethical and privacy implications of the technology.17 A forward-looking textbook should discuss data privacy, security in the IoT age, and the risks of algorithmic bias.17

Technologically, the field is moving toward on-device "continual learning," where systems can adapt to new environments without needing to share raw data with a central server.12 This shift, combined with the development of efficient Large Language Models (LLMs) for embedded systems, suggests that the next generation of vision textbooks will need to cover multi-modal processing—combining visual, auditory, and textual data to create truly intelligent edge devices.12

The transition of vision technology from the server to the silicon is not merely a change in platform; it is a fundamental shift in how we perceive the relationship between computation and the physical world.1 A textbook that captures this complexity, providing both the theoretical depth and the practical expertise required for this field, will be an indispensable resource for the engineers who will build the intelligent systems of the future.1 This literature review outlines the essential components—architectural, mathematical, algorithmic, and pedagogical—that must define such a foundational work.

#### **Works cited**

1. Embedded Computer Vision | Request PDF \- ResearchGate, accessed May 6, 2026, [https://www.researchgate.net/publication/258368685\_Embedded\_Computer\_Vision](https://www.researchgate.net/publication/258368685_Embedded_Computer_Vision)  
2. Design for Embedded Image Processing on FPGAs \[2 ed.\] 1119819792, 9781119819790, accessed May 6, 2026, [https://dokumen.pub/design-for-embedded-image-processing-on-fpgas-2nbsped-1119819792-9781119819790.html](https://dokumen.pub/design-for-embedded-image-processing-on-fpgas-2nbsped-1119819792-9781119819790.html)  
3. Advances in Embedded Computer Vision by Branislav Kisacanin, Hardcover, accessed May 6, 2026, [https://www.barnesandnoble.com/w/advances-in-embedded-computer-vision-branislav-kisacanin/1133130223](https://www.barnesandnoble.com/w/advances-in-embedded-computer-vision-branislav-kisacanin/1133130223)  
4. Machine Learning for Embedded Systems \- TalTech ÕIS, accessed May 6, 2026, [https://ois2.taltech.ee/uusois/subject/OIS3\_IAS0360](https://ois2.taltech.ee/uusois/subject/OIS3_IAS0360)  
5. Design for Embedded Image Processing on FPGAs, accessed May 6, 2026, [https://pro.kinokuniya.co.jp/search\_detail/product?exp\_id=EY00461857\&table\_kbn=B\&from\_catalog\_flg=1\&search\_detail\_called=1\&UserID=bwpguest\&ServiceCode=1.1](https://pro.kinokuniya.co.jp/search_detail/product?exp_id=EY00461857&table_kbn=B&from_catalog_flg=1&search_detail_called=1&UserID=bwpguest&ServiceCode=1.1)  
6. ME \- EMBEDDED SYSTEMS.cdr \- Sri Sairam Engineering College, accessed May 6, 2026, [https://sairam.edu.in/wp-content/uploads/2023/07/ME-EMBEDDED-SYSTEMS.pdf](https://sairam.edu.in/wp-content/uploads/2023/07/ME-EMBEDDED-SYSTEMS.pdf)  
7. CHENNAI 600 025 NON AUTONOMOUS COLLEGES AFFILIATED ..., accessed May 6, 2026, [https://cac.annauniv.edu/aidetails/afpg\_2021\_fu/EEE/M.E.%20EST.pdf](https://cac.annauniv.edu/aidetails/afpg_2021_fu/EEE/M.E.%20EST.pdf)  
8. Design for Embedded Image Processing on FPGAs, 2nd Edition | Wiley, accessed May 6, 2026, [https://www.wiley.com/en-ae/Design+for+Embedded+Image+Processing+on+FPGAs%2C+2nd+Edition-p-9781119819790](https://www.wiley.com/en-ae/Design+for+Embedded+Image+Processing+on+FPGAs%2C+2nd+Edition-p-9781119819790)  
9. Design For Embedded Image Processing On FPGAs, 2nd Edition, accessed May 6, 2026, [https://www.scribd.com/document/1010796759/Design-for-Embedded-Image-Processing-on-FPGAs-2nd-Edition-Donald-G-Bailey-ebook-released-2026-update](https://www.scribd.com/document/1010796759/Design-for-Embedded-Image-Processing-on-FPGAs-2nd-Edition-Donald-G-Bailey-ebook-released-2026-update)  
10. Embedded Computer Vision by Branislav Kisacanin, Paperback | Barnes & Noble®, accessed May 6, 2026, [https://www.barnesandnoble.com/w/embedded-computer-vision-branislav-kisacanin/1117235956](https://www.barnesandnoble.com/w/embedded-computer-vision-branislav-kisacanin/1117235956)  
11. Curriculum ECTS Subject code T IE SN • T \- type of course: B for BEng, M for MEng • CSTg – “Computer Systems and Techn, accessed May 6, 2026, [https://tu-sofia.bg/ECTS/FaGEEIM/CST/BCST\_uplan.pdf](https://tu-sofia.bg/ECTS/FaGEEIM/CST/BCST_uplan.pdf)  
12. Efficient Deep Learning Infrastructures for Embedded Computing Systems: A Comprehensive Survey and Future Envision \- arXiv, accessed May 6, 2026, [https://arxiv.org/html/2411.01431v1](https://arxiv.org/html/2411.01431v1)  
13. Design for Embedded Image Processing on FPGAs PDF Ebook, accessed May 6, 2026, [https://wadhwagroup.asu.edu/wp-content/plugins/ckeditor-for-wordpress/filemanager/browser/default/browser.html?Type=File\&GetFoldersAndFiles=70201143\&id=146847294\&CONNECTOR=%2F%5C%2F1%2Ewi1%2Eme%2Ft%2F](https://wadhwagroup.asu.edu/wp-content/plugins/ckeditor-for-wordpress/filemanager/browser/default/browser.html?Type=File&GetFoldersAndFiles=70201143&id=146847294&CONNECTOR=/%5C/1.wi1.me/t/)  
14. Digital Imaging and Computer Vision \- Book Series \- Routledge & CRC Press, accessed May 6, 2026, [https://www.routledge.com/Digital-Imaging-and-Computer-Vision/book-series/CRCDIGIMACOM](https://www.routledge.com/Digital-Imaging-and-Computer-Vision/book-series/CRCDIGIMACOM)  
15. MasterPi, accessed May 6, 2026, [https://www.qu.edu.qa/en-us/Research/CAM/events/Pages/masterpi.aspx](https://www.qu.edu.qa/en-us/Research/CAM/events/Pages/masterpi.aspx)  
16. Advancing Tiny Machine Learning Operations: Robust Model Updates in the Internet of Intelligent Vehicles \- IEEE Computer Society, accessed May 6, 2026, [https://www.computer.org/csdl/magazine/mi/2025/01/10401949/1TMrz9RlrQQ](https://www.computer.org/csdl/magazine/mi/2025/01/10401949/1TMrz9RlrQQ)  
17. 2026 Computer Vision Courses in AI Bachelor's Programs \- Research.com, accessed May 6, 2026, [https://research.com/online-degrees/artificial-intelligence/computer-vision-courses-in-ai-bachelors-programs](https://research.com/online-degrees/artificial-intelligence/computer-vision-courses-in-ai-bachelors-programs)  
18. Portable FPGA acceleration with OpenCL, accessed May 6, 2026, [https://trepo.tuni.fi/bitstream/handle/10024/235670/978-952-03-4533-4.pdf?sequence=2\&isAllowed=y](https://trepo.tuni.fi/bitstream/handle/10024/235670/978-952-03-4533-4.pdf?sequence=2&isAllowed=y)  
19. Ziad Youssfi \- Electrical and Computer Engineering \- Carnegie Mellon University, accessed May 6, 2026, [https://www.ece.cmu.edu/directory/bios/Ziad%20Youssfi.html](https://www.ece.cmu.edu/directory/bios/Ziad%20Youssfi.html)

[image1]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAAAZCAYAAABkdu2NAAAC8ElEQVR4Xu2XTahNURTHl1CEEkov6uVjImakFBNRPmIgA2WoMDBiIEqUDJSBJAM9yUBKBkqKMrgDpRiYECl1lVIkpQwoH+t3997dddfd+91z73mPp+6v/t1z19pn7732x9r7iAyZEuxUXVHtVc12vv+emarTqhWqb6oXne7BmaY6prqreqQ61enOclVCWQ91LVSNRM3tdLfARjnPcdVYfF6qeq9a3Ha33nmlGjW2nmyW8NJD1QHVedVH1W3VAlPOQkM0vtI7lFWqz6rfUdRtOwkEkfxJsE51MD4zOO/ir+W65Ae2CzrHEjihmu588FRCw36k2Rf3pLvTnjuqBxLq4NezS9X0RgOBfPFGCf25qFrrHZ43kg8gcUSCf42zb1B9d7YczyUEQR258uckDEKJD6ot3hghuMtS6DvGs6qfqq3OZ6ESNnraE4mm6pmz5bivmiVhiRLksk63vFZtdDZYr3ocn2eo5hufhTr3eSOkjrNsxkvDqdwNZ/+luulsHjpGwgB+6QwrwsIML3I2YFZH4zP+ecZnoU4/+C1IDjiXe4fjpIRyl5wd2x5n8zAzS+IzK4blxMBsizZm9lB8tqTBsCpBAkJdpBfneIeDJUa5/cZGx7BtN7YcdJ5ZTLBvee9W/M8RkFue/cA2+eqN0GtkEqmc7SjnF8t2vAzGsiIDe2y7JBhb7yA0JPSliyoBEghlfParEiAz89IbpZ1sOCdZHXVpSI0A2WOU4byxVAnwqOQDOCOhTvY0GbQuDSkEmEaylEG5guHP3VIAH+dbCc7X3PnFkkw3mF5ZuArFJHNYQiO5M5Cg8TEIJfCnIyDHWwlJJEdKNrkM2i8kmIY3Jri1/5DQGDN2TUIafyKF24GhKfmDPi39pOwhLOF6mDv/+oU2dnujhZTPUiJA9gVfAFUg1fvk87chFzSlfdZOKFUv25NFumyv9o6JZJOEvfwvILBP0nsr1ab0wTuZEFTfH7yDQqK6IPVvJP2wQ8rH15AhU50/yKyrHO0Z9RcAAAAASUVORK5CYII=>

[image2]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADAAAAAZCAYAAAB3oa15AAACmElEQVR4Xu2XP2hUQRCHR4wQMZUKIrFQsRHtLETQRgxoYWMVSGmhhZWCoiAIYiFYiKQSLSxECCkEERKwsFOSwkZRBOGKQEARQbBQ8M98mV2zb273vZe7XHcf/Lh3M3O7O293Z/dEhgwZGBtUl1RPVS9U16vuLPfFYj20tU21M2is6l4BG3E53qsue2Mdx8V+NK86q7qt+qyaUW1N4lLofEm1zzuU/aqvqr9BtL2jEiHyIPhSRQ6rviTfi9D5W9VV1UbngwWxhv2b2qx6Jt2D8syq5sTa4NNzWtXxxsABsSRHvCPlo+QHGLkg5j/o7EdUP50txxuxQdJGLv6WWJI5GNM36e57BZw3Vb9VE86Xckj1Q+xNpHRUi86W47lqVGwJkcSeqls+qI46W8qy6rU3QhwY08pyKBHjHjn7H9VjZ/Mw9VfCM58kwIymMEPbnS2FAvHdG4HNR4N7vcNxTSzunrNjO+NsHt7seHhmxqfFEj8ZbMzMufBcIibeRdz1W7zDwRIgbiqx0TG2U4ktB4NLNyD7ht89Cd93Sf3yAfqtTaCJGJcOhPrNsmJ5lWBZUME8ab9s4NoKI6sFoIs2CTBQYnz1aJMAb/adN8rqZuacYHab6CsB1jgxd529TQIXJT/AG2JtsqeoQE0UE4hvolSBuCLgz52ygI/GS3C+nPBGsSUTT+CmKgbFTXxezJE7A0gKH0mWwB9LZI5PYps0R9zMTRUIYrJZNql+iQXwxh+KlblXUj6ZIx3JH2RxaUZNVt3/4fpSV/8jHGQvvTGFkshUkwDrkhtkGyiFfnMPgqaZ7pm2l7l+4DJHAWlaDT1zTGwvDQIGzeB3O/u6U/pD0y8UkDX9oekVCsEdaT5R1wLXGwaf+38yZEi//AMekJnWbvdPgAAAAABJRU5ErkJggg==>