export default function generatePDFFiltro(data, username, filtros) {
    try {
        const content = data.rows.map((item) => {
            return [
                { text: item.id.toString(), fontSize: 8 },
                { text: item.cnpj, fontSize: 8 },
                { text: item.nome_empresa, fontSize: 8, alignment: "left" },
                {
                    text: new Intl.NumberFormat('pt-BR', { style: 'currency', currency: 'BRL' }).format(item.val_solicitado.toFixed(2)), fontSize: 8, alignment: "right"
                },
                {
                    text: item.created_at ? new Date(item.created_at).toLocaleDateString('pt-BR') : '', fontSize: 8, alignment: "center"
                },
            ];
        });

        const total = data.total_records;
        const totalField3 = data.rows.reduce((acc, item) => acc + parseFloat(item.val_solicitado || 0), 0);

        const dataFormatada = new Date().toLocaleDateString('pt-BR');
        const horaFormatada = new Date().toLocaleTimeString('pt-BR');

        const filtrosFormatados = JSON.stringify(filtros, null, 2)
        .replace(/\\u([\dA-F]{4})/gi, (_, code) => String.fromCharCode(parseInt(code, 16))) // Converte Unicode para UTF-8
        .replace(/[\{\}"]/g, '')  // Remove chaves e aspas extras
        .replace(/,/g, '\n')     // Adiciona quebra de linha para cada item
        .replace(/\\/g, '');        // Remove todas as barras invertidas

        const docDefinition = {
            pageSize: "A4",
            pageOrientation: "landscape",
            header: function (currentPage) {
                if (currentPage === 1) {
                    return {
                        columns: [
                            {
                                image: base64_image_logo,
                                width: 200,
                                margin: [40, 10, 0, 0],
                            },
                        ],
                    };
                }
                return null;
            },
            footer: function (currentPage, pageCount) {
                return {
                    text: `Usuario: ${username}${" ".repeat(50)}${dataFormatada} às ${horaFormatada}${" ".repeat(50)}Página ${currentPage.toString()} de ${pageCount}`,
                    alignment: "right",
                    margin: [0, 10, 30, 0],// margin: [esquerda, topo, direita, baixo]
                    fontSize: 7,
                };
            },
            content: [
                { text: "RELATÓRIO DE CONTATO POR PERÍODO", alignment: "center", margin: [0, 50, 0, 0], fontSize: 12, bold: true },
                {
                    text: "FILTROS APLICADOS",
                    alignment: "center",
                    margin: [0, 20, 0, 0],
                    fontSize: 10,
                    bold: true
                },
                {
                    text: filtrosFormatados,
                    alignment: "center",
                    margin: [0, 5, 0, 10],
                    fontSize: 10,
                    width: "auto" // Ajusta automaticamente para quebrar linha
                },
                {
                    table: {
                        widths: [40, 105, 280, "auto", "auto",],
                        body: [
                            [
                                { text: "ID", fontSize: 10, fillColor: "#bbf7d0", alignment: "center" },
                                { text: "CPF/CNPJ", fontSize: 10, fillColor: "#bbf7d0", alignment: "center" },
                                { text: "NOME", fontSize: 10, fillColor: "#bbf7d0", alignment: "center" },
                                { text: "VALOR SOLICITADO", fontSize: 10, fillColor: "#bbf7d0", alignment: "center" },
                                { text: "DATA", fontSize: 10, fillColor: "#bbf7d0", alignment: "center" },
                            ],
                            ...content,
                        ],
                    },
                    layout: 'lightHorizontalLines', // Mantém bordas leves para melhor visualização
                    alignment: "center", // Centraliza a tabela na página
                    margin: [80, 10, 0, 10], // margin: [esquerda, topo, direita, baixo]
                },
                {
                    text: `Total linhas: ${total}`+" ".repeat(30)+`Total Solicitado: ${new Intl.NumberFormat('pt-BR', { style: 'currency', currency: 'BRL' }).format(totalField3.toFixed(2))}`,
                    margin: [0, 20, 0, 0],// margin: [esquerda, topo, direita, baixo]
                    fontSize: 9,
                    alignment: "center",
                },
            ],
        };

        pdfMake.createPdf(docDefinition).open();
    } catch (error) {
        console.error("Erro ao gerar o PDF:", error);
    }
}