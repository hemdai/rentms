'use client';
import { useEffect, useState } from "react";
import PropertyListItem from "./PropertyListItem";
import apiServices from "@/app/services/apiServices";

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
        apiServices.get('/property/all-property').then((response) => {
            setProperties(response)
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