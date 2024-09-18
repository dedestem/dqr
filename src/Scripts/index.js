const { invoke } = window.__TAURI__.tauri;


document.addEventListener("DOMContentLoaded", (event) => {
    const popup = document.getElementById("popup");
    const ptext = document.getElementById("ptext");
    popup.style.visibility = "hidden";

    
    document.getElementById('Generate').addEventListener('click', async () => {
        const data = document.getElementById('input-link').value;
        if (!data) return;

        try {
            // Roep de Rust functie aan om de QR-code te genereren
            const imagePath = await invoke('generate_qr_code', { data });

            // Log de pad naar het bestand
            console.log('Image Path:', imagePath);

            // Toon de QR-code in de <img> tag
            document.getElementById('Generated').src = imagePath;
        } catch (error) {
            console.error('Error generating QR code:', error);
        }
    });

    document.getElementById('Export').addEventListener('click', async () => {
        document.getElementById('Generated').src = "";
        const downloadpath = await invoke("get_downloads_path");
        console.log(downloadpath);

        const data = document.getElementById('input-link').value;
        if (!data) return;

        try {
            // Roep de Rust functie aan om de QR-code te genereren
            const imagePath = await invoke('generate_qr_code_export', { data });

            // Log de pad naar het bestand
            console.log('Image Path:', imagePath);
            console.log('Exported')

            popup.style.visibility = "visible";
            ptext.innerText = "Exported: " + downloadpath + "/qr_" + data + ".svg";

            setTimeout(() => {
                popup.style.visibility = "hidden";
            }, 5000);
        } catch (error) {
            popup.style.visibility = "visible";
            ptext.innerText = 'Error generating QR code:', error;

            setTimeout(() => {
                popup.style.visibility = "hidden";
            }, 5000);
        }
    });
});
