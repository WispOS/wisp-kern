;; Automated system tasks like filesystem and such
(defv system-services 
 ;; native-driver is a representation of a driver the kernel itself has to handle.
 (native-driver "fat32")
 (native-driver "vga-disp")
 (native-driver "ps2-keyb"))