: local ( git---local )
    
    commit -1 0 OR . \true
    GitHub -1 0 OR . \true
    GitLab -1 0 OR . \true
    add -1 0 OR . \true

: block ( block---trackers )
    cookies -1 0 AND \ ---------|
    trackHTTP -1 0 AND \--------|--- everybody is false
    trackerHTTPS -1 0 AND \-----|
