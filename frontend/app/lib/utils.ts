import countries from "world-countries";
import apiServices from "../services/apiServices";
// upload country

    const uploadCountries = async () => { 
      const countryDetails = countries.map(country => ({
        name: country.name.common,   // Country Name
        iso: country.cca2,       // ISO 3166-1 alpha-2 code
        currency: Object.values(country.currencies || {})[0]?.name || "N/A", // Currency Name
        phone_code: country.idd?.root + (country.idd?.suffixes?.[0] || "") // Phone Code
    }));

    for (const country of countryDetails) {
        await apiServices.post("/country/create-country", JSON.stringify(country)).then((res) => {
            console.log(res, 'success');
        }).catch((err) => {
            console.log(err, 'error');
        });
    }

}