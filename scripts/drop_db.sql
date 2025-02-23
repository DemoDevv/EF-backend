DO $$ DECLARE
    r RECORD;
BEGIN
    -- Boucler sur chaque table dans le schéma public
    FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public') LOOP
        EXECUTE 'TRUNCATE TABLE public.' || r.tablename || ' CASCADE';
    END LOOP;
END $$;
