;; Automated system tasks like filesystem and such
(defv system-services 
 ;; When run in the user workspace this will run a user service
 (run-lisp-service "check-lan-machines-alertness"))