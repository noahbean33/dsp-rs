# **Definitive Framework for Embedded Digital Signal Processing Education: A Comprehensive Literature Review of Pedagogical Content and Technical Requirements**

The conceptualization of a contemporary textbook on embedded digital signal processing (DSP) requires a departure from traditional, purely mathematical treatments of the subject. While classical DSP texts focus heavily on the abstract manipulation of sequences, an embedded-centric approach demands a holistic integration of hardware constraints, real-time software engineering, and architectural awareness. The transition from continuous-time theories to discrete-time implementations is no longer sufficient; the modern engineer must navigate the intricacies of word-length effects, power consumption, and the orchestration of data movement within a silicon environment. The evidence gathered from current university syllabi and leading academic publications suggests that a definitive textbook in this domain must be structured around several core pillars: foundational discrete-time theory, micro-signal architecture, fixed-point numerical integrity, real-time software optimization, and practical application-driven project modules.

## **Evolution of the Embedded DSP Paradigm**

The historical trajectory of DSP education has moved through three distinct phases: the theoretical phase, the specialized processor phase, and the modern integrated-system phase. Initially, signal processing was a branch of applied mathematics, with little concern for the physical platform. This was followed by a period where specialized Digital Signal Processors (DSPs), such as the Texas Instruments TMS320 series, defined the curriculum.1 In this era, mastery of specific assembly languages and instruction sets was the primary metric of student success. Today, the field is characterized by a "hybrid" model where general-purpose microcontrollers with DSP extensions (e.g., ARM Cortex-M4) and open-source architectures like RISC-V have democratized the application of real-time signal processing.3

The curricular guidelines established by the ACM and IEEE in the CE2016 report reflect this evolution, identifying signal processing as a core knowledge area with a recommended allocation of approximately 30 core hours for undergraduate programs.6 The transition from the CE2004 standards to the CE2016 framework highlights an increasing emphasis on system-on-a-chip (SoC) design, field-programmable gate arrays (FPGAs), and security—all of which must be addressed in a comprehensive text.8

| Knowledge Area Identifier | Topic Description | Core Hours Recommended |
| :---- | :---- | :---- |
| CE-SGP-1 | History and Overview of Signal Processing | 1 |
| CE-SGP-2 | Relevant Tools, Standards, and Engineering Constraints | 3 |
| CE-SGP-3 | Sinusoidal and Transient Analysis | 4 |
| CE-SGP-4 | Convolution Operations | 2 |
| CE-SGP-5 | Transform Analysis (z-Transform, Fourier) | 5 |
| CE-SGP-6 | Frequency Response | 3 |
| CE-SGP-7 | Sampling and Reconstruction | 4 |
| CE-SPE | Systems and Specializations | Variable |

This structured breakdown indicates that the initial chapters of a textbook should not only cover the "how" of signal processing but also the "why" within an engineering context, explicitly addressing the constraints imposed by hardware and the tools used to manage them.9

## **Core Mathematical Foundations for Embedded Implementation**

While the mathematical underpinnings remain grounded in linear time-invariant (LTI) system theory, an embedded-focused textbook must frame these concepts within the limitations of a physical processor. The z-transform, for instance, serves as the primary tool for analyzing discrete-time systems, yet its practical utility in an embedded context is often realized through the derivation of difference equations that can be implemented in code.11

### **Time-Domain Signals and Systems**

The introductory narrative should emphasize the classification of signals, distinguishing between deterministic sinusoidal signals and stochastic random processes.11 A critical insight for the student is the realization that in an embedded system, time is a quantized commodity. The process of sampling, governed by the Nyquist-Shannon theorem, must be presented alongside the physical realities of anti-aliasing filters and the precision of the Analog-to-Digital Converter (ADC).4 The relationship between the sampling rate (![][image1]) and the processing time budget is the most vital constraint for any real-time system; the textbook must explicitly link the complexity of the chosen algorithm to the available clock cycles between interrupts.11

### **Frequency-Domain Analysis and the Fast Fourier Transform**

The Fast Fourier Transform (FFT) is the computational workhorse of frequency analysis. A textbook must go beyond the basic radix-2 decimation-in-time algorithm to explore the architectural support required for its efficient execution.1 This includes an analysis of bit-reversed addressing, a specialized hardware feature that reorders memory access to facilitate butterfly computations, and the use of pre-computed twiddle factor look-up tables to conserve processing cycles.1 Furthermore, the text should address the necessity of windowing functions (e.g., Hamming, Blackman, or Hanning) to mitigate spectral leakage, providing a clear comparison of the trade-offs between main-lobe width and side-lobe attenuation.11

## **Micro-Signal Architecture and Hardware Acceleration**

A profound understanding of the underlying silicon is what distinguishes an embedded DSP engineer from a general software developer. The literature indicates that the Blackfin processor and the ARM Cortex-M series are the two dominant architectures for educational purposes.4 A comprehensive text must detail how these architectures facilitate high-throughput signal processing.

### **The Blackfin and TMS320 Architectural Models**

The Blackfin processor, characterized by its Micro Signal Architecture, represents a convergence of a high-performance RISC processor and a specialized DSP.11 Key architectural elements that must be documented include:

* **Dual Multiply-Accumulate (MAC) Units:** Allowing for parallel computation of sum-of-products, which is the foundational operation for filtering and transforms.11  
* **Hardware Loops:** Dedicated registers that manage loop counters and zero-overhead branches, eliminating the typical software overhead associated with "for" or "while" loops.11  
* **Harvard Architecture:** The separation of instruction and data buses to allow for simultaneous memory access, crucial for maintaining the pipeline's throughput.1  
* **Circular Buffers:** Modulo addressing that allows delay lines (essential for FIR filters) to be managed without physically shifting data in memory, significantly reducing the computational load.1

### **The Emergence of ARM Cortex-M4 and RISC-V**

In recent years, the ARM Cortex-M4 has become a staple of embedded curricula due to its low cost and ubiquitous presence in the IoT market.4 The presence of a floating-point unit (FPU) and SIMD (Single Instruction, Multiple Data) instructions allows for a transition from high-level C code to highly optimized routines through the CMSIS-DSP library.4 Simultaneously, the RISC-V P-extension (Packed SIMD) is gaining traction in academic research, providing an open-source platform for students to explore the design and validation of DSP-specific instruction sets.5

| Platform | Processor Architecture | Notable DSP Extensions | Development Board Example |
| :---- | :---- | :---- | :---- |
| **Blackfin 533/537** | Micro Signal Architecture | Dual MAC, Video Instructions, Parallel ALU 11 | BF537 EZ-KIT |
| **TMS320C55x** | Traditional DSP | Dual 17x17 MACs, Specialized Adders 1 | C55x DSK |
| **ARM Cortex-M4F** | RISC with DSP Extensions | SIMD, FPU, CMSIS-DSP 4 | STM32F4 Discovery / Nucleo |
| **RISC-V (RV32IP)** | Open ISA | Packed SIMD (P-Extension) 5 | Custom FPGA Implementations |

The choice of platform influences the entire software stack. For instance, the Blackfin uses the VisualDSP++ IDE, while the ARM platforms typically rely on Keil MDK-ARM or Code Composer Studio (CCS).1 A textbook should ideally present these concepts in a way that allows for cross-platform learning, emphasizing the architectural patterns rather than just specific toolsets.

## **Numerical Integrity: The Challenge of Fixed-Point Arithmetic**

One of the most difficult transitions for students is moving from the infinite-precision world of MATLAB simulations to the finite-word-length world of embedded hardware. A textbook must provide a rigorous treatment of numeric representation, quantization, and overflow management.1

### **Fixed-Point Data Representation and Q-Notation**

Most embedded DSP tasks are performed using fixed-point arithmetic to save energy and silicon area. The student must master Q-notation (e.g., ![][image2]), where a 16-bit integer is used to represent a fractional value between \-1.0 and 0.9999.11 The textbook must explain the causal relationship between the chosen word length and the resulting noise floor of the system. The Signal-to-Quantization Noise Ratio (![][image3]) is a critical metric here, typically estimated as ![][image4] for a ![][image5]\-bit system.11

### **Managing Quantization and Overflow**

Real-time implementations face three primary numerical hazards:

* **Coefficient Quantization:** When filter coefficients (like those for an IIR filter) are truncated to fit into a 16-bit register, the poles of the filter may shift, potentially leading to instability or a failure to meet the desired magnitude response.11  
* **Rounding and Truncation Errors:** Each multiplication in a sum-of-products creates a result with more bits than the original operands (e.g., two 16-bit numbers produce a 32-bit product). The way these bits are managed—through rounding or truncation—introduces varying levels of noise into the signal chain.11  
* **Overflow and Saturation:** If the result of an addition exceeds the register's range, standard integer wraparound can create catastrophic distortions. The textbook should cover the implementation of saturation logic, where the processor clamps the value at its maximum or minimum limit, and the use of guard bits in accumulators to handle intermediate overflows.1

## **Real-Time Software Engineering and Optimization**

The software development lifecycle for embedded DSP is inherently different from general application development. A textbook must guide the reader through a hierarchical strategy that balances development speed with execution efficiency.3

### **The Multilevel Implementation Strategy**

The literature suggests a progression that begins with high-level modeling and ends with cycle-accurate optimization.11

1. **MATLAB/LabVIEW Prototyping:** Validating the mathematical logic and filter specifications in a high-level environment.13  
2. **Generic C Implementation:** Writing readable, portable C code that can be tested on a PC or simulator.1  
3. **Compiler-Assisted Optimization:** Utilizing compiler intrinsics (special C functions that map directly to assembly instructions), loop unrolling, and inlining to improve performance without writing raw assembly.11  
4. **Assembly Refinement:** Writing hand-optimized assembly for the most critical bottlenecks, such as the inner loop of an adaptive filter or the butterfly stage of an FFT.11

### **Data Movement and the Role of DMA**

In a real-time system, the CPU should not be burdened with moving data from the ADC to memory or from memory to the DAC. The Direct Memory Access (DMA) controller is a vital component that a textbook must explain in depth.4 Topics should include:

* **Autobuffer Mode:** Setting up DMA to automatically refill or empty circular buffers without CPU intervention.11  
* **Interrupt Synchronization:** How the completion of a DMA transfer triggers a high-priority interrupt, signalling the CPU that a new "block" of data is ready for processing.4  
* **Memory Management:** The trade-offs between using on-chip L1 SRAM (fastest, but limited) and off-chip SDRAM (larger, but slower), and how to use the Memory Management Unit (MMU) or cache systems effectively.11

## **Filter Design and Implementation in Real-Time**

Digital filters are the quintessential application of DSP. A text must cover Finite Impulse Response (FIR) and Infinite Impulse Response (IIR) filters not just as transfer functions ![][image6], but as real-time algorithms.11

### **FIR and IIR Architectures**

For FIR filters, the textbook should demonstrate implementation using both the direct form and the transposed form, highlighting why the transposed form is often preferred in hardware with multiple accumulators.11 For IIR filters, the narrative must pivot toward the use of Second-Order Sections (SOS), also known as biquads. This is a critical insight: higher-order IIR filters implemented as a single direct-form structure are notoriously sensitive to coefficient quantization and can easily become unstable.11

### **Adaptive Filtering and the LMS Algorithm**

Adaptive filters represent an advanced but necessary topic for modern systems. The Least Mean Squares (LMS) algorithm is the most common pedagogical example, used for applications like acoustic echo cancellation and system identification.1 The textbook should walk the student through the iterative adjustment of weights (![][image7]) based on an error signal (![][image8]), demonstrating how the filter "learns" the characteristics of an unknown environment.2

| Filter Type | Typical Application | Key Implementation Challenge | Code Complexity |
| :---- | :---- | :---- | :---- |
| **FIR (Fixed)** | Low-pass, Band-pass | Large memory for high order 11 | Low |
| **IIR (Fixed)** | Equalizers, Notch | Instability, limit cycles 11 | Medium |
| **Adaptive (LMS)** | Echo cancellation | Convergence speed vs. stability 1 | High |

## **Practical Application Modules and Case Studies**

To ensure that the theory is grounded in reality, a textbook must include a series of hands-on projects. These projects should span various domains, reflecting the breadth of modern embedded DSP applications.1

### **Audio and Voice Signal Processing**

Audio is an ideal entry point because of its low frequency range and the ability for students to "hear" the results of their algorithms. A textbook should include:

* **Graphic Equalizers:** Building a bank of IIR filters to control different frequency bands.11  
* **Audio Effects:** Implementing delay lines for reverberation, chorus, and flanging.11  
* **Speech Enhancement:** Using adaptive filters to remove background noise from a voice signal.1  
* **Dual-Tone Multi-Frequency (DTMF) Detection:** The classic touch-tone telephone problem, which teaches signal generation and spectral identification using the Goertzel algorithm.1

### **Embedded Image and Video Processing**

As embedded processors have gained power, image processing has become a standard topic. Key areas include:

* **2D Filtering:** Using convolution kernels for edge detection (Sobel, Canny) and noise reduction (Gaussian blur).5  
* **Color Space Conversion:** Translating between RGB and YCbCr formats for efficient video processing.1  
* **Image Compression:** A simplified introduction to the Discrete Cosine Transform (DCT) as used in the JPEG standard.1

### **Ultrasound Imaging: A Specialized Case Study**

A sophisticated textbook should provide at least one deep dive into a complex industrial application. Ultrasound imaging is a premier example, involving high-speed data acquisition and real-time processing.19 The literature highlights the use of the Hilbert Transform to derive an analytical signal from raw ultrasound pulses. This technique allows for downsampling the signal without aliasing while maintaining vital phase information, thereby reducing the data transfer bandwidth required for the FPGA back-end.19

## **The Modern Frontier: TinyML and Edge AI**

The most significant shift in the last five years is the integration of Machine Learning (ML) into embedded systems. A modern DSP textbook is incomplete without a chapter on TinyML—the deployment of ML models on microcontrollers with limited memory and power.20

### **Pre-processing for TinyML**

In many Edge AI applications, DSP is the critical "front-end." For instance, an audio recognition system does not feed raw waveforms into a neural network; instead, it uses a DSP stage to generate Mel-frequency cepstral coefficients (MFCCs) or spectrograms.21 The textbook must explain this pipeline:

1. **Sensing and Signal Conditioning:** Standard DSP filtering and gain control.21  
2. **Feature Extraction:** Reducing the raw data into a compact set of features that represent the signal's information content.21  
3. **Inference:** Running a quantized neural network (using TensorFlow Lite for Microcontrollers or similar) on the feature set.16

### **Optimization Techniques for Edge Intelligence**

The constraints of TinyML (often \<1MB of RAM and \<100MHz clock speeds) require specific model optimization techniques that align with DSP principles:

* **Quantization:** Converting 32-bit floating-point weights to 8-bit integers, paralleling the fixed-point DSP strategies.21  
* **Pruning:** Removing insignificant connections in a neural network to reduce memory footprint.21  
* **Hardware-Software Co-design:** Designing custom accelerators (often implemented on FPGAs or as ISA extensions) that can perform the matrix multiplications required by AI models more efficiently than a standard CPU.20

| Feature | Traditional DSP | TinyML (Edge AI) |
| :---- | :---- | :---- |
| **Data Source** | Continuous sensor streams | Segmented "events" or frames |
| **Logic Type** | Determinist algorithms (e.g., FFT) | Probabilistic inference |
| **Primary Goal** | Signal transformation/fidelity | Classification/Prediction |
| **Memory Constraint** | Buffer-limited (Kb) | Model-limited (Mb) |

## **Pedagogical Frameworks and Distance Learning**

The final requirement for an effective textbook is a consideration of how the material is taught. Modern education is moving toward flexible, decentralized models.24

### **Remote Laboratories (RLs)**

Because physical DSP hardware (like the TI C6711 DSK or the Agilent 54603B oscilloscope) can be prohibitively expensive to maintain for large classes, many universities are adopting remote laboratories.24 A textbook should be designed to support these environments, which typically include:

* **R-DSP Server Architecture:** A web-based interface that allows students to upload code and control lab equipment remotely.24  
* **Virtual Instrumentation:** Using LabVIEW or specialized GUIs to provide real-time visual feedback (oscilloscopes, spectrum analyzers) from remote hardware.24  
* **Interoperability:** Ensuring that the lab exercises can be performed across different platforms and accessed through standard web browsers.24

### **Challenges of Online DSP Education**

Distance learning in a lab-heavy field like embedded DSP presents unique challenges. The scholarship identifies two primary categories of difficulty: technical virtualization (how to accurately model or remotely access hardware) and "ambiance" (creating a collaborative environment that mimics a physical laboratory).26 The textbook must address these by providing self-contained, simulation-ready projects and clear, step-by-step guides for hardware setup.3

## **Synthesis of Requirements for the Ideal Textbook**

Drawing from the collective evidence of current research and academic practice, the ideal textbook for embedded signal processing must be an interdisciplinary guide. It is not enough to teach the "math" of ![][image9] without teaching the "silicon" of how that coefficient ![][image10] is stored in a ![][image2] register and how the multiplication is scheduled in a dual-MAC pipeline.

The narrative of the book should follow the flow of a real project:

* **Phase 1: The Design.** Mastering the LTI theory and frequency analysis required to define the system's goals.  
* **Phase 2: The Implementation.** Navigating the architectural constraints of the target processor and the numerical hazards of fixed-point arithmetic.  
* **Phase 3: The Optimization.** Utilizing DMA, SIMD, and hand-optimized assembly to ensure the system meets its real-time deadlines within a limited power budget.  
* **Phase 4: The Intelligence.** Integrating modern Edge AI techniques to turn signal processors into intelligent autonomous systems.

By adopting this structure, a textbook can transform signal processing from a collection of abstract formulas into a powerful toolkit for the next generation of embedded engineers. The convergence of DSP, micro-signal architecture, and machine learning is the new baseline for the industry, and educational resources must evolve to meet this multidimensional challenge. The focus remains on the causal link between mathematical choice and physical consequence—a link that is the hallmark of the expert embedded practitioner.

#### **Works cited**

1. Real-Time Digital Signal Processing: Implementations and Applications, accessed May 6, 2026, [https://books.google.com/books/about/Real\_Time\_Digital\_Signal\_Processing.html?id=QIj9Pthp\_T8C](https://books.google.com/books/about/Real_Time_Digital_Signal_Processing.html?id=QIj9Pthp_T8C)  
2. Real-Time Digital Signal Processing, accessed May 6, 2026, [https://dsp-book.narod.ru/RTDSP.pdf](https://dsp-book.narod.ru/RTDSP.pdf)  
3. College of Engineering Biographical Data \- Minh N. Do \- University of Illinois, accessed May 6, 2026, [https://minhdo.ece.illinois.edu/biography/MinhDo\_BioData.pdf](https://minhdo.ece.illinois.edu/biography/MinhDo_BioData.pdf)  
4. Digital Signal Processing Using Arm Cortex M Based Microcontrollers , Yifeng Zhu (book) www.staff.ces.funai.edu.ng, accessed May 6, 2026, [https://www.staff.ces.funai.edu.ng/papersCollection/browse/fetch.php/Digital\_Signal\_Processing\_Using\_Arm\_Cortex\_M\_Based\_Microcontrollers.pdf](https://www.staff.ces.funai.edu.ng/papersCollection/browse/fetch.php/Digital_Signal_Processing_Using_Arm_Cortex_M_Based_Microcontrollers.pdf)  
5. RISC-V Processor Design for DSP Tasks | PDF | Parallel Computing \- Scribd, accessed May 6, 2026, [https://www.scribd.com/presentation/867752668/Major](https://www.scribd.com/presentation/867752668/Major)  
6. Computer Engineering Curricula 2016 \- ACM, accessed May 6, 2026, [https://www.acm.org/binaries/content/assets/education/ce2016-final-report.pdf](https://www.acm.org/binaries/content/assets/education/ce2016-final-report.pdf)  
7. Launching curricular guidelines for computer engineering: CE2016, accessed May 6, 2026, [https://www.computer.org/csdl/proceedings-article/fie/2016/07757433/12OmNzwHvbD](https://www.computer.org/csdl/proceedings-article/fie/2016/07757433/12OmNzwHvbD)  
8. The IEEE Computer Society and ACM's Collaboration on Computing Education \- IEEE Xplore, accessed May 6, 2026, [https://ieeexplore.ieee.org/iel7/2/7433333/07433351.pdf](https://ieeexplore.ieee.org/iel7/2/7433333/07433351.pdf)  
9. Setting the Stage for CE2016 \- s2.SMU, accessed May 6, 2026, [https://s2.smu.edu/\~mitch/ftp\_dir/pubs/fie14.pdf](https://s2.smu.edu/~mitch/ftp_dir/pubs/fie14.pdf)  
10. Chapter 2 Computer Engineering as a Discipline, accessed May 6, 2026, [https://www.ccny.cuny.edu/sites/default/files/2023-10/ACM-IEEE--computer-engineering-2016--excerpts.pdf](https://www.ccny.cuny.edu/sites/default/files/2023-10/ACM-IEEE--computer-engineering-2016--excerpts.pdf)  
11. Embedded Signal Processing with the Micro Signal ... \- download, accessed May 6, 2026, [https://download.e-bookshelf.de/download/0000/5690/34/L-G-0000569034-0002357146.pdf](https://download.e-bookshelf.de/download/0000/5690/34/L-G-0000569034-0002357146.pdf)  
12. Real-Time Digital Signal Processing: Fundamentals, Implementations and Applications, accessed May 6, 2026, [https://www.barnesandnoble.com/w/real-time-digital-signal-processing-sen-m-kuo/1115394948](https://www.barnesandnoble.com/w/real-time-digital-signal-processing-sen-m-kuo/1115394948)  
13. Real-Time Digital Signal Processing from MATLAB to C with the ..., accessed May 6, 2026, [https://www.routledge.com/Real-Time-Digital-Signal-Processing-from-MATLAB-to-C-with-the-TMS320C6x-DSPs/Welch-Wright-Morrow/p/book/9780367736453](https://www.routledge.com/Real-Time-Digital-Signal-Processing-from-MATLAB-to-C-with-the-TMS320C6x-DSPs/Welch-Wright-Morrow/p/book/9780367736453)  
14. Digital Signal Processing: A Practical Guide for Engineers and Scientists by Steven Smith | eBook | Barnes & Noble®, accessed May 6, 2026, [https://www.barnesandnoble.com/w/digital-signal-processing-steven-smith/1111305549](https://www.barnesandnoble.com/w/digital-signal-processing-steven-smith/1111305549)  
15. Digital Signal Processing Using Arm Cortex M Based Microcontrollers \_ Cem Unsalan,H. Deniz Gurhan,M. Erkin Yucel (2024) www.staf, accessed May 6, 2026, [https://www.staff.ces.funai.edu.ng/textbook-solutions/browse/download/Digital\_Signal\_Processing\_Using\_Arm\_Cortex\_M\_Based\_Microcontrollers.pdf](https://www.staff.ces.funai.edu.ng/textbook-solutions/browse/download/Digital_Signal_Processing_Using_Arm_Cortex_M_Based_Microcontrollers.pdf)  
16. Introduction to TinyML. In this blog post, you'll learn about… | by Ayyuce Demirbas | Medium, accessed May 6, 2026, [https://medium.com/@ayyucedemirbas/introduction-to-tinyml-685b9bef1377](https://medium.com/@ayyucedemirbas/introduction-to-tinyml-685b9bef1377)  
17. Embedded Signal Processing with the Micro Signal Architecture \- ni, accessed May 6, 2026, [https://download.ni.com/pub/gdc/tut/exp\_isbn\_978\_0\_471\_73841\_1.pdf](https://download.ni.com/pub/gdc/tut/exp_isbn_978_0_471_73841_1.pdf)  
18. Embedded Signal Processing | PDF | Digital Signal Processor \- Scribd, accessed May 6, 2026, [https://www.scribd.com/document/22889540/Embedded-signal-processing](https://www.scribd.com/document/22889540/Embedded-signal-processing)  
19. Real-Time DSP for Ultrasound Imaging | PDF | Analog To Digital Converter \- Scribd, accessed May 6, 2026, [https://www.scribd.com/document/466476673/C141StudPap-libre](https://www.scribd.com/document/466476673/C141StudPap-libre)  
20. TinyML: Applications, Algorithms, Co-design and Implementations \- MyImpact \- Newcastle University, accessed May 6, 2026, [https://myimpact.ncl.ac.uk/UploadFiles/303665/8f4b4e4b-c477-49c0-adda-0bb572fb7f3f.pdf](https://myimpact.ncl.ac.uk/UploadFiles/303665/8f4b4e4b-c477-49c0-adda-0bb572fb7f3f.pdf)  
21. A Systematic Review of State-of-the-Art TinyML Applications in Healthcare, Education, and Transportation \- IEEE Xplore, accessed May 6, 2026, [https://ieeexplore.ieee.org/iel8/6287639/10820123/11266886.pdf](https://ieeexplore.ieee.org/iel8/6287639/10820123/11266886.pdf)  
22. Presentations \- RISC-V Summit Europe 2025, accessed May 6, 2026, [https://riscv-europe.org/summit/2025/presentations](https://riscv-europe.org/summit/2025/presentations)  
23. RISC-V Summit Europe 2026 \- Posters, accessed May 6, 2026, [https://riscv-europe.org/summit/2026/posters](https://riscv-europe.org/summit/2026/posters)  
24. A Remote Laboratory for Real-Time Digital Image Processing on Embedded Systems, accessed May 6, 2026, [https://www.researchgate.net/publication/40422955\_A\_Remote\_Laboratory\_for\_Real-Time\_Digital\_Image\_Processing\_on\_Embedded\_Systems](https://www.researchgate.net/publication/40422955_A_Remote_Laboratory_for_Real-Time_Digital_Image_Processing_on_Embedded_Systems)  
25. (PDF) eDSPLab: Remote laboratory for experiments on DSP applications \- ResearchGate, accessed May 6, 2026, [https://www.researchgate.net/publication/235287985\_eDSPLab\_Remote\_laboratory\_for\_experiments\_on\_DSP\_applications](https://www.researchgate.net/publication/235287985_eDSPLab_Remote_laboratory_for_experiments_on_DSP_applications)  
26. Transforming Computer Engineering Laboratory Courses for Distance Learning and Collaboration\*, accessed May 6, 2026, [https://ecasp.ece.iit.edu/publications/2012-present/IJEE%20Distance%20Learning.pdf](https://ecasp.ece.iit.edu/publications/2012-present/IJEE%20Distance%20Learning.pdf)

[image1]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABEAAAAZCAYAAADXPsWXAAABDElEQVR4XmNgGFHAA4h3APEKdAliASsQvwPibCDeDMSWqNKEASMQzwfiIiBuAOL/QKyErIAYIA7Ed4HYhQHiIhCfZBDNALFdEF2CFDCJAWIIC7oEMUAAiCWB+A4DxBAQG4RB4iSDT0D8DV2QVAByxR50QVIAKHpBhoDChWwAihGQIaAYIhuA0sYTIJZBl4ACTSAOAWJmdAlkAEqloPDgRpcAAn4g7meAGDKBAeJ1DMALxIcZcHtlIQPEAJA6eTQ5OADlj+cMECdjAxkMkPACYU40OYbXUIlyKI0NnAPi7VA2yDJQ2KEAmOmgsuMKmhwMgIqFOijbB4iFkeTAIIABEpha6BJogAeKRwEBAADKmio70gVilwAAAABJRU5ErkJggg==>

[image2]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADIAAAAZCAYAAABzVH1EAAACNklEQVR4Xu2Wz0tVQRzFv2KCUiLoKhS0MCFMIgLbB7kRCcJFUDtBXQYFoiQI0j/QJoggClokLVwYSLVo1SbITRAEEUUrhYTIRWLlOW9m9Dvf++u9xXu94H7ggHfOd8Y5c++bGZGSkobTAt2EVqFX0GJs/x9chD5AL6Ep6Da0Ca1A3arO0gfdso1Vwr7HbKPnMtSunrug6+o5wSD0HlqC2mKrAr2/4t5WYAT67dupL8orguOx7564vsdj+4AwdtAGNBRVGD6KK0wLQebE+WdUG9/QGNTjvVqCcKXZN4ybFWRX3BfxHVqHWmP7EK7wsrjVuWQ8zXloB3pgDU+tQQJFQR7ZhizCBJm2w3iaUJc18D8P8k3cQL3WMMyIq7trDU+9gnyCPkOT0LS4xRyPKjzhR3TEGoaH4uquWcNTryDb0Kh63vJtCUKQIv5IfuB6BbHwi2A9P/WIaoJw8qz5ZQ1Fo4KE+glrVBOE6VlzzxqKegS5Ar2D+lVbZhCe4jTCQfdD3AZwwz/Pe18PlkZeEE7yqG305AXhjsXfxFnV9kRc/QnVVmHWG+egYeiUb38ODXiPYYvICsIxOZkXkn4NyQtyVZK7qV34CJ7mPEFZ8Bi67//mdSB06JTkOcN/zro0BXg3WoMWJN4oGNr2oRhM80bc3J6K262K7nyVV8/vjiG4M7yF7iifp/9J9dwoeCW5IG5evJHXzGtxK/QV+in515em5pnErzzrMtn08LfB3eK05Nw4S0pKmp99GKKhwEJ+JYsAAAAASUVORK5CYII=>

[image3]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADoAAAAYCAYAAACr3+4VAAADHklEQVR4Xu2Wz6tNURTHl1BEfkSkFAllQlJKDAwQ+TFAUYqBgYkJipLSSwZm8msgBpT8AYiBdDMSAwZESj0SRVLKAPmxPnft9c4+6+1zbkoGr/upb/ectdbZd/9Ya+8t0qfPiGKUaqGqo7qimpXsx1Uz0nNknOq86onqsmpz3T2MiWLtomli/5kzNqnEFKm+dTX1q5W7qt+qvaoB1XfVTrGB08HIGdVP1SnVdtVtse83yfABAJNySywG/VKtr0WIbEi+XM7bgi/XbtX4oegCk1X3VfML9geqrcF+WKzhRcHu4PsYjRm7VM/E4j4HH7BSg9GYYAL57n10KB/EfE0Z0V2xfOZyjkh9NbeJrcSOzBbxzJgeHYmHqi1SrdDMulv2ibVRYpnYN0ejQ3kl5iOmyGtpHijp6bDi71Q3pD1FqG/aWxIdCVaTbDgnFkd55FxTnQw2h2xgotdEh/JD2ie4m56e5y/EOjG6FmGQZsSQ0m14HTbNrA9ijOq66ptqReXu9mFe9u5MVT0Sa596zyHT+M9DwV6DIDYVHyziPa6a+3rh9VeqYTpL2jpsRsSyuk5pIECGfJEqbX0XvirWxspk7wkdoxEfUD7TNIiNuupF24SQenEHz+NJu1JawiWxuNmZjWds7Ph/zSSxOqGBg8nmm0AnvTdBFvgklchr3vFNiZJYJeW0BdKWuLjaX5O9EVaQg76EryArkL93PKAANcesU8tLgw9IvZfRKJY1TMw9aV8Z/p+NKOKbUCPUymA0JpjVT1LV2QSxxphVB9/q7J32qG1qvgST1olGsSy4I9Y+nS7h52epdLC3DrQjFhBvMXPEZp7fnKdi8RzIp6XamTlrGTS+Y8lW4rkMv3w4pC3fsxGVaDo/qfc40LPZcxdWjFQ4IPXbBB0q3ViWi9lPqG5m9jdi18Wmq98C1R6xzjwWu5+W4koDoR7XSXU2c93ke59kyiUfKL616XmIxemX2eQyflHsHCp1IoeLP9dA4vnlOpZvIKzw/4Syov8XVBuD75/CbO5Pz3OlfviPKHzXQ6WUHzFw7pI6va6Gffr06VPjD7aUyQ/O3xYPAAAAAElFTkSuQmCC>

[image4]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIkAAAAZCAYAAAAFQg2KAAAFH0lEQVR4Xu2ZW6huUxSAh1Dkfonc2pFITi65pVxObpF4wHGINw9KyoNC52mfBw90HiR5cEmUXFISSicPf55EkSJyqXNKx4MHEcqRy/j2WOP/xxprrvVf9nH2xvxq9O81xpzrMuaYY445t0ilUqlUKv9bjlI5riDHquwf2lXK7CPmrw0qFwX94Y0+ywGhzVS4+XkqT6lsTbYhDlW5S+VJlU3JFqEN975f5aRki/yk8teAfKFy67j12nCWyj1ZOcC3Kldn5QDuq1tUDky2aRwsE189H/T8nX0ZZbfK6ePWBc5R+UbljOb6OpXPVI4etyhzocq7Koc11/z+0Oidm1W+C9c44A8xJ/RlhoPEXpx7Z94Rs21M+llYyoo5uEraTh21rMPkAYnyWGi3TeVPmfjlZbE2i0BQxCABAmikslMsi0QYO57F2HQ4RuVTlUuba7/R9yqnNboSRDgDdm3S85FvhutPVF4I1/upPCP2QrcFfeQUMfvD2aA8KGbjdx78uxblBLHA9Jk6isYByNBkPyZFlKelPcGA+76arv+JIEH4O+PP6yxBO6T7InzYvkmX8cGiVogQpfF+vzfXnqXAg2AUdBGCg+xDu8wHYn1PzYYprDZIIkPvnmHGxm+HJZXPkw5/5uzNYJUGcxYWCRImON/WsaHECATGtOBwfI3LN/yw0TtvSDuFwlCQ4Ji3G8kRTfDSr5gSp7BWQcJ3Z5+SLcgmjr+bfzPX2a/T4DnUMBub63mDxH2bE8ZK6kf5q1ixyhJyiVhdQbQPMZJykPTpHV7mCbGBLhVzrP0E1SNiH41Qx/wslvUolBdhrYIkc6bKnUmHz8m4H8nkvl6XPdBc9+GD+17QMZbo+oKE5Y93cP9+LOZzdkPcrwWdPHrioN6rsj3pMiPp9hvSO+ygflFZlsILKVvE+r8o7TX8a7Hg/TcHiddjeYm+QeyeDNRNQc+SNC1rMtF2SXv5Jau8Lv1BQr35nEx8+4rYJCQgO/71ICGKIwwkej6oj5GUg6FPD7w8tm3ZEKAWoU0Jf6+XsiFxpXQLRZyCc7Ieeci6zcyiQUIGLX2bB8lrYoHk+JJ+dtBFWLbxV6nAn3e5ga/EnreUDSiZ2REfjNIW1MFWCoZRoy/VE0TqfdJdoyNePJWgCMTGdn2I9RokO6T8bZ5d86B6kHAkUcL7lXZ6iwTJ42LPW076wSAZJX3EP+CQpKdCLzmC8xKe40sMWeXIiXkMfeO5SoRdAnaeMS/rYblhAmRfg2eEPKjuYzJNiT0dJL5jzf1WTgPzoF7Q6O4OOta8y8L1xSq/SXebyolpdgSHaxSkEVJkfpmh85EllS/F6pJzk20W9laQkO0oOjO+SdiZDQ3LYtk59qUuo0/foaafVeVlChYJkvfFntc5vyqtk3eIFU0nBh0FFO38ZfoO02gTD9N8cHOKL82AG8X685uhesd2fTbMyN4IEnYuLGnbpTsIXA8FyQaxCXty0OXjhBIUrkwc+jvu874g4b5HtE0rJQDPQuIB3xiO5QkCzjTYDv/YNq/A9mx3VooNNh++pfm9om0ePzgL/UiX4CewfcJzSbnxrGVeVhskvgSXxMG5b4n5Is9sP/YeqqfY9tPmWbFtKcfyxQFL3C42ZgTZLpXNYqfc/n58uy9dfYKPmYBD9aJcI1a4bJJyuuyDtvShL/dYr6w2SPYEj6qcn5UJ/vlJpt0q5SOCPphAl8vkfzLHN3/nDURlCrPMykqlUqlUKpVKpVKp/Of4G3Y1mbMo+OgXAAAAAElFTkSuQmCC>

[image5]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAYCAYAAADzoH0MAAAA4UlEQVR4Xu2SsQ4BQRCGR/AAKESiVUg8ArVCq5CoJWqPovQIotWrNApR6L2ASHSI8P+Zvcvd2Nvr5b7kyyUzs5PZ2RP5O6qwCVseG4m6TBbwE/ANt3F1gLvoAUsP3uBJcibi4bMNgho8wCvsmlwMi9hgYxNgAB9wLbovL1P4gn0Tb4s23pv4D0vRe87g2LmDTziC5bjSA+/F+/E1LB3RCVY2kYTjs8iOHxE9ZybcMAsqNuHIbcDxswrYNNigJJq8mDgZii6WE/I1UvAg/7C5aIOj6ML4//M7cXFad2cKClJ8Afb8NOnZGfs/AAAAAElFTkSuQmCC>

[image6]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACsAAAAZCAYAAACo79dmAAACFklEQVR4Xu2WPyhHURTHj1BEUZSB+hU2BuXPwoiBslgMBmVgsBmsFrtkkpLBpgwyKMMri1j9G5Ey2BQD+XO+3Xe5vr97f+/34/e296lv/e4573fvuefec94TycjIo0bVyMY/8qAaYGMCFaorVY4dPtZUnx41x362Q0exzwVBTrOxSLbEP2eQNzGB+JgQ45tjR8y86p2NJYDsrqp62RHCZs3HipjNDLFDaVCdxPoPCHRdTOAFwZEj0AN2KO1i7uKO+CfCEYY2UiqIYYqNTL+YB5FBJukKXKjuVW3scED2R51xnWrQGVuwziYbXapUu6oP1Z5qg4SsYhJbcAx8oawDu1mrY9W5+Dd3GysIgkB2XlV3HtlFQsEUygba4r6qOh4PqyIJt7cz1RMbXXC8WBAT+YDvhY0O8C+xMcClqo+NDpHqmY0uuAJYEIXE4G7Bh8yHKDbYcVUPG4lIEoJFIFgQgTHYgL2TIYoJdkbMEScRSUKw9k76KNRfLfjvNhtjcM8XxXQDl2tVN9lAwQJDJ8BiqHimXsxOkflQJwAoiEjM88yCGD/qokNVqRoRU5BYm7Fz/SInPxl1NSvmTYKjYF+oALfEn/0m1WT8G3fVdpbD7yfygR+tLjXwxsEiy2RHDSCTFrSxFmfM4GRuVK1kLzvl+pDpYkca4MhPVbXsKBIE+SjhF08qpP7xXU7Q+Ev9VBxTdbIxI8PhC+api+HYVouRAAAAAElFTkSuQmCC>

[image7]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACkAAAAZCAYAAACsGgdbAAACRElEQVR4Xu2Wv0scURDHR0xAMZIYJWphcoogCYKRqCBopxAL0xiIoH+ClU3+iHSSKgQsBREkhdpYHDYKgpWSoAgnSNJFIlhY+OP75b2XnZ27veTuthC8D3y5ffN2Z+e9mTe3IlWqlMUQNGWNJfAF2rTGNHkBHUI1dqIEHkNb0EM7kRZX0Jw1lkE9dAR12IlKeQCdQp12okxuJJ0Fx/gIzVhjBaxAl9YY4Ba3WqNhBOpW4zpoDRpUNk0z9BKq9WPW23to4O8d+cyL2808eKqeQhfQgbLvSvQA08pr7aAPOocalS3wDZoV5+Ma2oFe+Tmmc1XcIi1vxMXxSBufQaP+mgHoIHkzFeAJ5gsDY37coGyBbXGZoT/6ZQcITEJZMYF4uqBfYrIaVsNJvpAvJmwndL7ox4TOuTN6XDA1EgXAef0M/dLnV2XTtEMn/jcPFj+dNfkxf/kCfShYL9p5sSBJWKh+hjv0HZpWNk1ikKw3nqoFZWO9MdWskcAy9E6N/xVkoYW+FZexpJaVGGQ4ADogtpasxOvms8T/VQoWucJmh7Cnhrpmt2hRcyTRZ4g+tBIGwoCyEt3MAzbsrwNM3bEUWLWHaWb96YVxZ3PexndYJqRIdtrETf4Wd7oY8B9xf3ln0Kfo1hhJzTwD/YR6jH1dnM8f0GszR1h2uqPk8QTqlejEs/k+F9eUk+DuLomraw13KmNshHb6S/qQyInLYOqk+YGxL660Uoenns4rZRz6YI1pcuc/egN7UL81/icb4r4fqtxvbgHQj24a+14NwQAAAABJRU5ErkJggg==>

[image8]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACQAAAAYCAYAAACSuF9OAAAB7ElEQVR4Xu2WzytmYRTHj1BkSqLxYz9JiYUxzWIWNlZigyj+AAtrZO8fsFFT02QhpdnNZmokrMTCxo8NCyJRUooS4vv1PFfnnve5Y+57N+T91Lf3Pt9z733O85xz731FCrwTFqHv1kzBCdRrzXwphdahahtIwQB0as18+AhtQeU2kAed0Lg10/IHOrJmBh6gQWum4Qr6Zc0M3ELz1tT8gO6gOWgP+hwPP62o23hkBtqBLqAv4ibh8bG4a9h3IfhwcJFBuqBD6Ksf8ybcjRI/roCuoXY/1qyKa3JOvg1NiOuzWmgX6ng+M86suGty4EUMFCmPKx1R43rowP9qaqA2cdfyHr9NjAmGdpWwqYMJ0WSpmqE+aBm6l/hWJyVU7FUl7j5DKsZEWZJW5WmCCX3w5kvNmpRQBBPZEJdYBJ/KnAkV/0xoygYM3P5zqMkGxJXrp5cuOxv7xh9zHgvnzEmI0Jw2Hsu1AlUqj+eF+oEluRRXogg+BDyfu8BW2FSxiMSnjE19psYt0BJUpzzCCSaNR1guxnS5yrzHBfyFxlQsYl9c0wfhVn+DPtmAYk3Cb2ruRqM1xSWV1HOECfdYMw2jklDzPGBP8bvI72MmhsV9GLPAlybfV7o/M/Gq/g8RrmzBmv9JA9Qv8ddDgbfLI8mRYN/h2TGxAAAAAElFTkSuQmCC>

[image9]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAHYAAAAZCAYAAADkBdqeAAADZElEQVR4Xu2aTahNURTHl1DKZ8hHKe+9JKKQrxQzFGJiqEwMTEwYUMzoFTMZIcJAiDLAzOCViQwUkVIGJAaKUhTysX53n909b9n73Hvee/eec887v/r37l3r3vvue/+z1l573ytSUwTbbaCmt9mt2qk6ZhM1vc9aKcDYKapZNjgCNqg+2mAbLFa9Uk2wiQpRiLHnVH8DmpvkbRw9THJpXqtO2WCbbFTttcEKUYixnl/iTAuxR1zuoE0kbFMdssGcfFJdssGKUKixvhpDDIozfrNNKJNUN1T9NpGTK6ovNlgRCjOWtoupD2xCGRC3dl6X8DrIG45dEHnwv2emTVQAjD1ug91gvThzqExLqzbMxfDdBg0MaKukeWFMVk1tphtwn7Wbx1WJD9Lshp9V84anOwet9I7qj+qu6qIRVcSb8sOU5avqkQ2moL2mB6/bqvfDHtGE6m/Vsrg45qgW5tC0xjPHGRj2UvVT9S4gb0ioDQMXRGhK9lwWV41ULVui3xKfgPdJ6wFqhrgL0L7PLJ1sPHOcQYvFuK02kUAuq9WSv2aDAZarntmggVOadl6rk2zpMUWhDWPOgE2IqzRyVHSMdo19rtplg4YyGJteNnpBUTCNB9hhBjCbHBNxDPKtzKAbbLLBAGUw1lZE2RUly/ms/avnm2rIBlM8FXds6Jkv8ZZcD09x+LvZvbQFEzGmhs54+WcMiavo2EQMb1RvbTCB1ntPdVTcFoc390TiFwKDE1Vb8z/3JXtJbEAF2V6NDojbTFOFNhcbrmIHFLfEGQqLpPk6DFEhaPtcYGWortmqq+K2alR8WejqMsXa+UNc9afhoIEq9axR9aXuW/xBSBnA2HWqmzKOjYWx+hDgrA0WDP/I0Rg7Udw3J0KDKZ3Rn7ItSycy6LqxVOYLGflxGWsva3WfiRfNaIxdqnqs2i9upjiSyrHc+KWJgeh8KpdF142FKn7QHjLWTtxpMa0DyxLHpv3JfUwNzRa0/FC8dKwWt5bmgWnPD1llI2SsPUtP63TyGMzi4AeD/Vm8HQoXSOvj05oOETK2Hdiy7UhucwbAWQBD5uEktkRctcIK1Znkdk2XwFi2anlZKW7KBz4gYft4QtyWkaWHrxFdEPehB7cZPmt6BFqvHyYZLpmQYXryk5nCr8ljwj9HOAJe3A95RgAAAABJRU5ErkJggg==>

[image10]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAoAAAAZCAYAAAAIcL+IAAAArklEQVR4XmNgGAW0AsJArAnEzOgSMOANxIeAWAKIWYF4PhD/B+IiZEVmQPweiLWRxGyA+B8QuyCJgXX+RhYAglYg3grEHDABFgaIwqswAQaIJEhROZIYgzgDROFSJDEZIH7CgGatJANEoS+SGMhakBjIZCUoZuCBCiIrBHkMJAYC0QwQ54FBHhD/AuJXQHwbiOWB+AoDRMMsmCIYAFkjxoAIaFBYggKfEa5iFOACALXEHEzF6LAAAAAAAElFTkSuQmCC>