;; Useful for startup banners in the user console.
;; Or for spawning long runnign plugins like file indexer for a prompt plugin
(on-startup 
 (@supervisor-spawn console-file-indexer))

(prompt "~> ")

;; Useful for saving console history or pruning it or rotating and such 
(on-shutdown ())