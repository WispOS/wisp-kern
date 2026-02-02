;; Automated system tasks like filesystem and such
(defv system-services 
 ;; run-native-driver is a representation of a driver the kernel itself has to handle.
 (run-native-driver "fat32")
 (run-native-driver "vga-disp")
 (run-native-driver "ps2-keyb")
 ;; When run in the user workspace this will run a user service
 (run-lisp-service "")

 )


(defv users
 `(able))
