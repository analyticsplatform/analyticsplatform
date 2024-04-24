'use client'

import React, { useState } from 'react';
import DeckGL from '@deck.gl/react';
import { TileLayer } from '@deck.gl/geo-layers';
import { BitmapLayer, LineLayer } from '@deck.gl/layers';
import { MapViewState } from '@deck.gl/core';

const INITIAL_VIEW_STATE: MapViewState = {
  longitude: -2.5, // Centered on the UK
  latitude: 54,
  zoom: 5,
  pitch: 0,
  bearing: 0,
};

const MapPage = () => {
  const [selectedFilter, setSelectedFilter] = useState('');
  const [viewState, setViewState] = useState<any>(INITIAL_VIEW_STATE);

  // Sample data for the map
  const data = [
    { sourcePosition: [-0.118092, 51.509865], targetPosition: [-1.548567, 53.801277] },
    { sourcePosition: [-1.548567, 53.801277], targetPosition: [-2.994362, 53.400002] },
    { sourcePosition: [-2.994362, 53.400002], targetPosition: [-3.188267, 55.953252] },
  ];

  const layers = [
    new TileLayer({
      data: 'https://c.tile.openstreetmap.org/{z}/{x}/{y}.png',
      minZoom: 0,
      maxZoom: 19,
      tileSize: 256,
      renderSubLayers: (props: any) => {
        const {
          bbox: {west, south, east, north}
        } = props.tile;

        return new BitmapLayer(props, {
          data: undefined,
          image: props.data,
          bounds: [west, south, east, north]
        });
      }
    }),
    new LineLayer({
      id: 'line-layer',
      data,
      getSourcePosition: (d: any) => d.sourcePosition,
      getTargetPosition: (d: any) => d.targetPosition,
      getColor: [0, 128, 255],
      getWidth: 2,
    }),
  ];

  const handleFilterChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    setSelectedFilter(event.target.value);
  };

  const handleResetMap = () => {
    setViewState(INITIAL_VIEW_STATE);
  };

  return (
    <div className="flex h-screen">
      <div className="relative w-full">
        <DeckGL
          viewState={viewState}
          onViewStateChange={({ viewState }) => setViewState(viewState)}
          controller={true}
          layers={layers}
        />
        <button
          className="absolute top-4 right-4 bg-white text-black px-4 py-2 rounded-md shadow-md"
          onClick={handleResetMap}
        >
          Reset Map
        </button>
      </div>
    </div>
  );
};

export default MapPage;
