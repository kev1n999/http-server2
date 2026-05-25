const btn = document.getElementById('calcBtn');
const result = document.getElementById('result');

btn.addEventListener('click', async () => {
    const x = parseInt(document.getElementById('x').value, 10);
    const y = parseInt(document.getElementById('y').value, 10);

    if (Number.isNaN(x) || Number.isNaN(y)) {
        result.textContent = 'Digite números inteiros válidos';
        return;
    }

    try {
        const response = await fetch('/sum', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ x, y })
        });

        const data = await response.text();
        result.textContent = `Resultado: ${data}`;
    } catch (err) {
        result.textContent = 'Erro ao chamar API';
        console.error(err);
    }
});