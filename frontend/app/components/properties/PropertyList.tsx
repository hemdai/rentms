'use client';
import { useEffect, useState } from "react";
import PropertyListItem from "./PropertyListItem";

export type PropertyType = {
    id: string;
    title: string;
    description: string;
    price_per_night: number;
    bedroom: number;
    bathroom: number;
    guest: number;
    image: string;
};
const PropertyList = () => {
    const [properties, setProperties] = useState<PropertyType[]>([]);
    const getProperties = async () => {
        const url = 'http://localhost:8080/property/all-property';
        await fetch(url, {
            method: 'GET',
            headers: {
                Authorization: 'Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImhlbWRhaUBoZW1kYWkuY29tIiwiaWF0IjoxNzQxMjU3NDE0LCJpZCI6MSwic3ViIjoiIiwibmFtZSI6IiIsInJvbGUiOiIiLCJleHAiOiIyMDI1LTA0LTA1VDEwOjM2OjU0LjczNzM0OVoiLCJpc192YWxpZCI6ZmFsc2V9.O-1zQ9h8ep5EfmkSznkXknY9BO5HzfwuNevSHFJkOB8' }
        }).then(response => response.json()).then((json) => {
            console.log('json', json);
            setProperties(json)
        }).catch((error) => {
            console.log('error', error);
        })
    };
    useEffect(() => {
        getProperties();
    }, []
    );
    return (
        <>
            {
                properties.map((property) => {
                    return <PropertyListItem
                        key={property.id}
                        property={property}
                    />
                })
            }
        </>
    )
}

export default PropertyList;