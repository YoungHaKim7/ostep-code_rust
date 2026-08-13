# ostep-code_rust
https://github.com/remzi-arpacidusseau/ostep-code

- Code from various chapters in OSTEP (http://www.ostep.org)

# Operating Systems: Three Easy Pieces
Remzi H. Arpaci-Dusseau and Andrea C. Arpaci-Dusseau
Arpaci-Dusseau Books
November, 2023 (Version 1.10) 


<table>
  <td width="1000pt" valign="top">

<center>
<p>
<table>

<tbody><tr>

<td bgcolor="yellow"><b>Intro</b> </td> 
<td bgcolor="#f88017"><b>Virtualization</b> </td> 
<td bgcolor="#f88017"><b></b> </td> 
<td bgcolor="#00aacc"><b>Concurrency</b> </td> 
<td bgcolor="#4CC417"><b>Persistence</b> </td> 
<td bgcolor="#3EA99F"><b>Security</b> </td> 

</tr> 


<tr>

<td bgcolor="yellow"><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/dedication.pdf" style="color:black">Dedication</a> </td> 
<td bgcolor="#f88017"><small>3</small> <i><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/dialogue-virtualization.pdf" style="color:black">Dialogue</a> </i> </td> 
<td bgcolor="#f88017"><small>12</small> <i><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/dialogue-vm.pdf" style="color:black">Dialogue</a> </i> </td> 
<td bgcolor="#00aacc"><small>25</small> <i><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/dialogue-concurrency.pdf" style="color:black">Dialogue</a> </i> </td> 
<td bgcolor="#4CC417"><small>35</small> <i><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/dialogue-persistence.pdf" style="color:black">Dialogue</a> </i> </td> 
<td bgcolor="#3EA99F"><small>52</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/dialogue-security.pdf" style="color:black"><i>Dialogue</i> </a> </td> 

</tr> 


<tr>

<td bgcolor="yellow"><a href="preface.pdf" style="color:black">Preface</a> </td> 
<td bgcolor="#f88017"><small>4</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/cpu-intro.pdf" style="color:black">Processes</a> </td> 
<td bgcolor="#f88017"><small>13</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/vm-intro.pdf" style="color:black">Address Spaces</a> <sup><a href="https://github.com/remzi-arpacidusseau/ostep-code/tree/master/vm-intro"><font color="black">code </font></a><font color="black"> </font></sup><font color="black"> </font></td> 
<td bgcolor="#00aacc"><small>26</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/threads-intro.pdf" style="color:black">Concurrency and Threads</a> <sup><a href="https://github.com/remzi-arpacidusseau/ostep-code/tree/master/threads-intro"><font color="black">code </font></a><font color="black"> </font></sup><font color="black"> </font></td> 
<td bgcolor="#4CC417"><small>36</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/file-devices.pdf" style="color:black">I/O Devices</a> </td> 
<td bgcolor="#3EA99F"><small>53</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/security-intro.pdf" style="color:black"><i>Intro Security</i> </a> </td> 

</tr> 


<tr>

<td bgcolor="yellow"><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/toc.pdf" style="color:black">TOC</a> </td> 
<td bgcolor="#f88017"><small>5</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/cpu-api.pdf" style="color:black">Process API</a> <sup><a href="https://github.com/remzi-arpacidusseau/ostep-code/tree/master/cpu-api"><font color="black">code </font></a><font color="black"> </font></sup><font color="black"> </font></td> 
<td bgcolor="#f88017"><small>14</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/vm-api.pdf" style="color:black">Memory API</a> </td> 
<td bgcolor="#00aacc"><small>27</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/threads-api.pdf" style="color:black">Thread API</a> <sup><a href="https://github.com/remzi-arpacidusseau/ostep-code/tree/master/threads-api"><font color="black">code </font></a><font color="black"> </font></sup><font color="black"> </font></td> 
<td bgcolor="#4CC417"><small>37</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/file-disks.pdf" style="color:black">Hard Disk Drives</a> </td> 
<td bgcolor="#3EA99F"><small>54</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/security-authentication.pdf" style="color:black"><i>Authentication</i> </a> </td> 

</tr> 


<tr>

<td bgcolor="yellow"><small>1</small> <i><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/dialogue-threeeasy.pdf" style="color:black">Dialogue</a> </i> </td> 
<td bgcolor="#f88017"><small>6</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/cpu-mechanisms.pdf" style="color:black">Direct Execution</a> </td> 
<td bgcolor="#f88017"><small>15</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/vm-mechanism.pdf" style="color:black">Address Translation</a> </td> 
<td bgcolor="#00aacc"><small>28</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/threads-locks.pdf" style="color:black">Locks</a> <sup><a href="https://github.com/remzi-arpacidusseau/ostep-code/tree/master/threads-locks"><font color="black">code </font></a><font color="black"> </font></sup><font color="black"> </font></td> 
<td bgcolor="#4CC417"><small>38</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/file-raid.pdf" style="color:black">Redundant Disk Arrays (RAID)</a> </td> 
<td bgcolor="#3EA99F"><small>55</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/security-access.pdf" style="color:black"><i>Access Control</i> </a> </td> 

</tr> 


<tr>

<td bgcolor="yellow"><small>2</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/intro.pdf" style="color:black">Introduction</a> <sup><a href="https://github.com/remzi-arpacidusseau/ostep-code/tree/master/intro"><font color="black">code </font></a><font color="black"> </font></sup><font color="black"> </font></td> 
<td bgcolor="#f88017"><small>7</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/cpu-sched.pdf" style="color:black">CPU Scheduling</a> </td> 
<td bgcolor="#f88017"><small>16</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/vm-segmentation.pdf" style="color:black">Segmentation</a> </td> 
<td bgcolor="#00aacc"><small>29</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/threads-locks-usage.pdf" style="color:black">Locked Data Structures</a> </td> 
<td bgcolor="#4CC417"><small>39</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/file-intro.pdf" style="color:black">Files and Directories</a> </td> 
<td bgcolor="#3EA99F"><small>56</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/security-crypto.pdf" style="color:black"><i>Cryptography</i> </a> </td> 

</tr> 


<tr>

<td bgcolor="yellow"></td> 
<td bgcolor="#f88017"><small>8</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/cpu-sched-mlfq.pdf" style="color:black">Multi-level Feedback</a> </td> 
<td bgcolor="#f88017"><small>17</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/vm-freespace.pdf" style="color:black">Free Space Management</a> </td> 
<td bgcolor="#00aacc"><small>30</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/threads-cv.pdf" style="color:black">Condition Variables</a> <sup><a href="https://github.com/remzi-arpacidusseau/ostep-code/tree/master/threads-cv"><font color="black">code </font></a><font color="black"> </font></sup><font color="black"> </font></td> 
<td bgcolor="#4CC417"><small>40</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/file-implementation.pdf" style="color:black">File System Implementation</a> </td> 
<td bgcolor="#3EA99F"><small>57</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/security-distributed.pdf" style="color:black"><i>Distributed</i> </a> </td> 

</tr> 


<tr>

<td bgcolor="yellow"></td> 
<td bgcolor="#f88017"><small>9</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/cpu-sched-lottery.pdf" style="color:black">Lottery Scheduling</a> <sup><a href="https://github.com/remzi-arpacidusseau/ostep-code/tree/master/cpu-sched-lottery"><font color="black">code </font></a><font color="black"> </font></sup><font color="black"> </font></td> 
<td bgcolor="#f88017"><small>18</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/vm-paging.pdf" style="color:black">Introduction to Paging</a> </td> 
<td bgcolor="#00aacc"><small>31</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/threads-sema.pdf" style="color:black">Semaphores</a> <sup><a href="https://github.com/remzi-arpacidusseau/ostep-code/tree/master/threads-sema"><font color="black">code </font></a><font color="black"> </font></sup><font color="black"> </font></td> 
<td bgcolor="#4CC417"><small>41</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/file-ffs.pdf" style="color:black">Fast File System (FFS)</a> </td> 
<td bgcolor="#3EA99F"></td> 

</tr> 


<tr>

<td bgcolor="yellow"></td> 
<td bgcolor="#f88017"><small>10</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/cpu-sched-multi.pdf" style="color:black">Multi-CPU Scheduling</a> </td> 
<td bgcolor="#f88017"><small>19</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/vm-tlbs.pdf" style="color:black">Translation Lookaside Buffers</a> </td> 
<td bgcolor="#00aacc"><small>32</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/threads-bugs.pdf" style="color:black">Concurrency Bugs</a> </td> 
<td bgcolor="#4CC417"><small>42</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/file-journaling.pdf" style="color:black">FSCK and Journaling</a> </td> 
<td bgcolor="#3EA99F"><b>Appendices</b> </td> 

</tr> 


<tr>

<td bgcolor="yellow"></td> 
<td bgcolor="#f88017"><small>11</small> <i><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/cpu-dialogue.pdf" style="color:black">Summary</a> </i> </td> 
<td bgcolor="#f88017"><small>20</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/vm-smalltables.pdf" style="color:black">Advanced Page Tables</a> </td> 
<td bgcolor="#00aacc"><small>33</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/threads-events.pdf" style="color:black">Event-based Concurrency</a> </td> 
<td bgcolor="#4CC417"><small>43</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/file-lfs.pdf" style="color:black">Log-structured File System (LFS)</a> </td> 
<td bgcolor="#3EA99F"><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/dialogue-vmm.pdf" style="color:black"><i>Dialogue</i> </a> </td> 

</tr> 


<tr>

<td bgcolor="yellow"></td> 
<td bgcolor="#f88017"></td> 
<td bgcolor="#f88017"><small>21</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/vm-beyondphys.pdf" style="color:black">Swapping: Mechanisms</a> </td> 
<td bgcolor="#00aacc"><small>34</small> <i><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/threads-dialogue.pdf" style="color:black">Summary</a> </i> </td> 
<td bgcolor="#4CC417"><small>44</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/file-ssd.pdf" style="color:black">Flash-based SSDs</a> </td> 
<td bgcolor="#3EA99F"><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/vmm-intro.pdf" style="color:black">Virtual Machines</a> </td> 

</tr> 


<tr>

<td bgcolor="yellow"></td> 
<td bgcolor="#f88017"></td> 
<td bgcolor="#f88017"><small>22</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/vm-beyondphys-policy.pdf" style="color:black">Swapping: Policies</a> </td> 
<td bgcolor="#00aacc"></td> 
<td bgcolor="#4CC417"><small>45</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/file-integrity.pdf" style="color:black">Data Integrity and Protection</a> </td> 
<td bgcolor="#3EA99F"><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/dialogue-monitors.pdf" style="color:black"><i>Dialogue</i> </a> </td> 

</tr> 


<tr>

<td bgcolor="yellow"></td> 
<td bgcolor="#f88017"></td> 
<td bgcolor="#f88017"><small>23</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/vm-complete.pdf" style="color:black">Complete VM Systems</a> </td> 
<td bgcolor="#00aacc"></td> 
<td bgcolor="#4CC417"><small>46</small> <i><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/file-dialogue.pdf" style="color:black">Summary</a> </i> </td> 
<td bgcolor="#3EA99F"><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/threads-monitors.pdf" style="color:black">Monitors</a> </td> 

</tr> 


<tr>

<td bgcolor="yellow"></td> 
<td bgcolor="#f88017"></td> 
<td bgcolor="#f88017"><small>24</small> <i><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/vm-dialogue.pdf" style="color:black">Summary</a> </i> </td> 
<td bgcolor="#00aacc"></td> 
<td bgcolor="#4CC417"><small>47</small> <i><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/dialogue-distribution.pdf" style="color:black">Dialogue</a> </i> </td> 
<td bgcolor="#3EA99F"><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/dialogue-labs.pdf" style="color:black"><i>Dialogue</i> </a> </td> 

</tr> 


<tr>

<td bgcolor="yellow"></td> 
<td bgcolor="#f88017"></td> 
<td bgcolor="#f88017"></td> 
<td bgcolor="#00aacc"></td> 
<td bgcolor="#4CC417"><small>48</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/dist-intro.pdf" style="color:black">Distributed Systems</a> </td> 
<td bgcolor="#3EA99F"><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/lab-tutorial.pdf" style="color:black">Lab Tutorial</a> </td> 

</tr> 


<tr>

<td bgcolor="yellow"></td> 
<td bgcolor="#f88017"></td> 
<td bgcolor="#f88017"></td> 
<td bgcolor="#00aacc"></td> 
<td bgcolor="#4CC417"><small>49</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/dist-nfs.pdf" style="color:black">Network File System (NFS)</a> </td> 
<td bgcolor="#3EA99F"><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/lab-projects-systems.pdf" style="color:black">Systems Labs</a> </td> 

</tr> 


<tr>

<td bgcolor="yellow"></td> 
<td bgcolor="#f88017"></td> 
<td bgcolor="#f88017"></td> 
<td bgcolor="#00aacc"></td> 
<td bgcolor="#4CC417"><small>50</small> <a href="https://pages.cs.wisc.edu/~remzi/OSTEP/dist-afs.pdf" style="color:black">Andrew File System (AFS)</a> </td> 
<td bgcolor="#3EA99F"><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/lab-projects-xv6.pdf" style="color:black">xv6 Labs</a> </td> 

</tr> 


<tr>

<td bgcolor="yellow"></td> 
<td bgcolor="#f88017"></td> 
<td bgcolor="#f88017"></td> 
<td bgcolor="#00aacc"></td> 
<td bgcolor="#4CC417"><small>51</small> <i><a href="https://pages.cs.wisc.edu/~remzi/OSTEP/dist-dialogue.pdf" style="color:black">Summary</a> </i> </td> 
<td bgcolor="#3EA99F"></td> 

</tr> 

</tbody></table> 

</p> 
</center> 

</td> 
</table>
