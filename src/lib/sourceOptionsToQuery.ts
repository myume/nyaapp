export const nyaaOptionsToQuery = (data: Record<string, string>) => {
  const components = [];
  if (data["Filter"]) {
    components.push(`f=${data["Filter"]}`);
  }

  if (data["Category"]) {
    components.push(`c=${data["Category"]}`);
  }

  if (data["query"]) {
    components.push(`q=${data["query"].replaceAll(" ", "+")}`);
  }

  return components.join("&");
};

export const optionsToQueryMap: Record<string, typeof nyaaOptionsToQuery> = {
  Nyaa: nyaaOptionsToQuery,
};
