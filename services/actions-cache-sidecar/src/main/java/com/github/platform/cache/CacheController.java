package com.github.platform.cache;

import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import org.springframework.beans.factory.annotation.Autowired;

import java.time.OffsetDateTime;

@RestController
@RequestMapping("/actions-cache")
public class CacheController {

    private final CacheService cacheService;

    @Autowired
    public CacheController(CacheService cacheService) {
        this.cacheService = cacheService;
    }

    @PostMapping("/store")
    public ResponseEntity<StoreResponse> store(@RequestBody StoreRequest request) {
        StoreResponse response = cacheService.storeObject(request);
        return ResponseEntity.status(response.getHttpStatus()).body(response);
    }

    @PostMapping("/restore")
    public ResponseEntity<RestoreResponse> restore(@RequestBody RestoreRequest request) {
        RestoreResponse response = cacheService.restoreObject(request);
        return ResponseEntity.status(response.getHttpStatus()).body(response);
    }
}
