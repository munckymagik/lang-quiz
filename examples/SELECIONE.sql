SELECIONE
  as.actividade,
  CONTE(*) COMO conta
DE horas_do_dia COMO hs
  JUNTE actividades COMO as EM (hs.actividade_id = as.id)
ONDE hs.tempo_de_lazer = VERDADEIRO
AGRUPE POR as.actividade
ORDENE POR conta DESCENDO;
