function gerarContraChequePDF(data) {
    const docDefinition = {
        pageSize: "A4",
      content: [
        {
            columns: [
                {
                    image: base64_image_logo,
                    width: 60,
                    absolutePosition: { x: 42, y: 42 }
                },
            ]
          },
          {

                layout: {
                  hLineWidth: function (i, node) {
                    return (i === 0 || i === node.table.body.length) ? 1 : 0;
                  },
                  vLineWidth: function (i, node) {
                    return (i === 0 || i === node.table.widths.length) ? 1 : 0;
                  },
                  hLineColor: function () {
                    return 'black';
                  },
                  vLineColor: function () {
                    return 'black';
                  }
                },
                table: {
                  widths: ['*'],
                  body: [
                    [
                        {
                          columns: [
                            { text: '', width: '5%' },
                            { text: 'ESTADO DO TOCANTINS', width: '50%', style: {
                                fontSize: 9,
                                bold: true,
                              },
                              margin: [30, 0, 0, 5] //// margin: [esquerda, topo, direita, baixo]
                            },
                            { text: 'Recibo de Pagamento', width: '50%', style:
                                {
                                    fontSize: 9,
                                    bold: true,
                                },
                            }
                          ]
                        },
                    ],
                    [
                        {
                          columns: [
                            { text: '', width: '15%' },
                            { text: 'FUNDO MUNICIPAL DA EDUCAÇÃO BRASILANDIA', width: '50%', style:
                                {
                                    fontSize: 9,
                                    bold: true,
                                },
                                margin: [-20, -5, 0, 5]
                                
                            },
                            { 
                                columns: [
                                    { text: 'Mês de Referência:', style:
                                        {
                                            fontSize: 9,
                                        },
                                        margin: [-50, -5, 0, 5] // margin: [esquerda, topo, direita, baixo]
                                    },
                                    { text: 'FEVEREIRO DE 2025', style:
                                        {
                                            fontSize: 9,
                                            bold: true,
                                        },
                                        margin: [-45, -5, 0, 5] // margin: [esquerda, topo, direita, baixo]
                                    }
                                ]
                            }
                          ]
                        },
                    ],
                    [
                        {
                          columns: [
                            { text: '', width: '15%' },
                            { text: 'Avenida Tibiriça Milhomem', width: '50%', style: 'defaultStyle',
                                margin: [-20, -5, 0, 5]
                                
                            },
                            { 
                                columns: [
                                    { text: 'Padrão de Importação:', style: 'defaultStyle',
                                        margin: [-50, -5, 0, 5] // margin: [esquerda, topo, direita, baixo]
                                    },
                                    { text: 'FEVEREIRO DE 2025', style:
                                        {
                                            fontSize: 9,
                                            bold: true,
                                        },
                                        margin: [-30, -5, 0, 5] // margin: [esquerda, topo, direita, baixo]
                                    }
                                ]
                            }
                          ]
                        },
                    ],
                    [
                        {
                          columns: [
                            { text: '', width: '15%' },
                            { text: 'CNPJ: 30793166000173', width: '50%', style: 'defaultStyle',
                                margin: [-20, -5, 0, 5]
                                
                            },
                            { 
                                columns: [
                                    { text: 'Impressão:', style: 'defaultStyle',
                                        margin: [-50, -5, 0, 5] // margin: [esquerda, topo, direita, baixo]
                                    },
                                    { text: '9 de Abril de 2025 às 09:35', style:
                                        {
                                            fontSize: 9,
                                            bold: true,
                                        },
                                        margin: [-75, -5, 0, 5] // margin: [esquerda, topo, direita, baixo]
                                    }
                                ]
                            }
                          ]
                        },
                    ],
                  ]
                },

          },
        {
            layout: {
                hLineWidth: function (i, node) {
                  return (i === 0 || i === node.table.body.length) ? 1 : 0;
                },
                vLineWidth: function (i, node) {
                  return (i === 0 || i === node.table.widths.length) ? 1 : 0;
                },
                hLineColor: function () {
                  return 'black';
                },
                vLineColor: function () {
                  return 'black';
                }
              },
          table: {
            widths: ['*', '*'],
            body: [
              ['Nº Matricula: 752', 'CPF: 70074379151'],
              ['PIS/PASEP: 166.41991.87-4', ''],
              ['Servidor: WILLIAM SANTOS COSTA', 'Cargo Atual: MOTORISTA C.B.O.: 782310'],
              ['Vínculo: TEMPO DETERMINADO', 'Departamento: FUNDO MUNICIPAL DA EDUCACAO'],
              ['Lotação: SECRETARIA M EDUCACAO MDE', 'Dt. Adm.: 17/02/2025'],
              ['MARGEM CONSIGNÁVEL:', '0']
            ],
            
          },
          margin: [0, 10, 0, 0]

        },

        {
            layout: {
                hLineWidth: function (i, node) {
                  return (i === 0 || i === node.table.body.length) ? 1 : 0;
                },
                vLineWidth: function (i, node) {
                  return (i === 0 || i === node.table.widths.length) ? 1 : 0;
                },
                hLineColor: function () {
                  return 'black';
                },
                vLineColor: function () {
                  return 'black';
                }
              },
          table: {
            widths: ['*'],
            body: [
              ['MARGEM CONSIGNÁVEL: 0']
            ]
          },
          margin: [0, 10, 0, 0]
        },
  
        {
          text: '\nProventos e Descontos',
          style: 'sectionHeader'
        },
        {
          style: 'tableExample',
          table: {
            widths: ['auto', '*', 'auto', 'auto', 'auto', 'auto'],
            body: [
              [
                'Código', 'Descrição', 'Parc.', 'Ref.', 'Proventos', 'Descontos'
              ],
              ['1', 'SALARIO CONTRATUAL', '-', '12,00', '650,57', '0,00'],
              ['106', 'SALARIO FAMILIA - CLT', '-', '1,00', '26,00', '0,00'],
              ['45', 'INSS', '-', '7,50', '0,00', '48,79']
            ]
          }
        },
  
        {
          text: '\nTotais',
          style: 'sectionHeader'
        },
        {
          layout: 'noBorders',
          table: {
            widths: ['*', '*', '*', '*'],
            body: [
              ['BASE PREVIDÊNCIA', 'BASE IRRF', 'Dep. IRRF', 'Bruto'],
              ['650,57', '0,00', '1,00', '676,57'],
              [{ text: 'Descontos', colSpan: 2 }, {}, 'Liquido:', '627,78']
            ]
          }
        },
  
        {
          text: '\nPara verificar a autenticidade desse contra cheque acesse:',
          margin: [0, 10, 0, 0]
        },
        { text: 'http://www.controlcid.com.br/folhaweb/autenticar-contra-cheque/', link: 'http://www.controlcid.com.br/folhaweb/autenticar-contra-cheque/', color: 'blue' },
        {
          text: 'Informe a Matricula, CPF e o código de verificação: 9939c.c06eb-75605',
          margin: [0, 5, 0, 0],
          fontSize: 10
        }
      ],
      styles: {
        header: {
          fontSize: 16,
          bold: true
        },
        subheader: {
          fontSize: 12,
          bold: true
        },
        sectionHeader: {
          fontSize: 11,
          bold: true,
          margin: [0, 10, 0, 5]
        },
        tableExample: {
          margin: [0, 5, 0, 15],
          fontSize: 10
        }
      },
      defaultStyle: {
        fontSize: 9
      }
    };
  
    pdfMake.createPdf(docDefinition).download('contra-cheque.pdf');
  }
  