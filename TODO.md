# Todo:
    - implement mqtt messages
    - implement better logging 
    - implement better readings
    - implement better operation or resources
    - implment automatic handle of new versions (dont know)
    - develop auto installation script for linux debian (checks version file and dowloads from the repo, also that installs necessary dependencies)

# Setup testing enviroment
- Debian system, such as a VM;

# Installation tool 
- Start with debian only;
    
- Enhance with more types of linux-based installations: different commands for different distros will be needed;
    
- If no packages could be retrived, suggest what repos to add and what specific packages are missing
    
- Provide `installation_log.txt` in the same folder where the installer is;
    
- Check if packages are present and search only for what is missing;
    
- Ask for different installation position, if any is specified install to `/server_watchdog/`;
    
- Enable daemon and configure system. 

# Base watchdog
- Interfaccia da shell tramite `server_wtdg`
    - `enable`/`disable`
    - `unistall`
    - `check updates` controlla la repository e il file di versionamento, in caso fetcha e mostra file README con updates alla fine.
    - `--help` per ogni comando 
    - `mqttest` per testare con messaggio sui diversi canali
        - `-l` loop ogni 2,5 sec
    - `status` rendere visualizzabile una breve struttura su come attualmente il servizio è configurato.

    - `components` rendere attivabili o disattivabili funzionalità tramite shell
        - `-s`: mostra stato del componente specificato a det. ID
        - `-l`: lista tutte le componenti con relativo ID assegnato
        - `-e`: attiva le componenti del relativo ID
        - `-d`: disattiva le compoennti del relativo ID
        - `-a`: aggiungi componente e nuovo ID (da sviluppare)
        - `-del`: rimuovi componente al relativo ID (da sviluppare, non eliminare componenti di default)
        - `` rendere modificabile il comportamento del programma delle funzionalità attive (es: unità di misura, etc)

# Future features:  
- "Componenti" (Feature da implementare successivamente)
Il programma voglio strutturarlo in "componenti", in modo che questi siano programmabili dall'utente, facilmente modificabili sia a livello di shell che di script e sviluppabili.
Inoltre voglio far sì che siano categorizzabili asseconda dei loro ID e Tags.
Le "Properties" dovranno essere strutture dati ben definite che il programma riesca a leggere e modificare direttamente da shell.

I file di implementazione dei componenti vanno aggiunti in un det. formato in una cartella detta "Addons".
    - Struttura componente, dentro ad `/server_watchdog/Addons/`:
        > `./CustomAddon/`
            > `./src/`............#source code (develop sample project and APIs)
            > `properties.wtg`..........#component configuration in standard format

Componenti "base" di default, non eliminabili:
    - Comp_1: 
        Fetch system resources usage and organize them in a nice format (eg: json), store them in a cached folder and trasmit a copy to MQTT channel `@server_statistics`.
        
    - Comp_2: 
        Free cache.

    - Comp_3: 
        - MQTT messages and channels.

    - Comp_4: 
        - Automatic optimizations.

    - Comp_5: 
        - Loop del daemon settato ogni 10 minuti per operazioni low_priority, ogni 5 per operazioni mid_priority, ogni 60 secondi per operazioni high_priority. Modificabile via shell.
    
    - Ogni parametro del programma modificabile via sintassi predefinita da me su un file nella cartella di installazione `config.wtd` o simile.

- Enable different log levels by shell for log on different components:
    eg: `server_wtdg -l 15`:
    - `1` stands for "debug infos";
    - `5` for "mqtt messages sent";
    - specifying both means our log file will include both in the same one;
    - Add separator with timestamp in logfile whenever log starts a new day or this command changes logs behaviour.